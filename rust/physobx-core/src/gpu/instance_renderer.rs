//! Instance renderer for GPU-instanced cube rendering

use super::camera::{Camera, CameraUniform};
use super::context::GpuContext;
use super::render_target::{OffscreenTarget, HDR_FORMAT};
use super::shadow::ShadowRenderer;
use bytemuck::{Pod, Zeroable};

/// Vertex data for a cube
#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x3,  // position
        1 => Float32x3,  // normal
    ];

    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

/// Instance data (position + rotation + color)
#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct InstanceData {
    pub position: [f32; 3],
    pub _padding: f32,
    pub rotation: [f32; 4], // quaternion (x, y, z, w)
    pub color: [f32; 3],
    pub _padding2: f32,
}

/// Shadow uniform data (light view-projection matrix)
#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct ShadowUniform {
    pub light_view_proj: [[f32; 4]; 4],
}

/// Instance renderer using GPU instancing
pub struct InstanceRenderer {
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
    camera_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    // Shadow bindings
    shadow_bind_group_layout: wgpu::BindGroupLayout,
    shadow_uniform_buffer: wgpu::Buffer,
    shadow_bind_group: Option<wgpu::BindGroup>,
    index_count: u32,
    max_instances: u32,
    half_extent: f32,
}

impl InstanceRenderer {
    /// Create a new instance renderer
    pub fn new(ctx: &GpuContext, max_instances: u32, half_extent: f32) -> Self {
        // Create shader module
        let shader = ctx.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Cube Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/cube_instance.wgsl").into()),
        });

        // Create cube geometry
        let (vertices, indices) = create_cube_geometry(half_extent);
        let index_count = indices.len() as u32;

        let vertex_buffer = ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        // Instance buffer (will be updated each frame)
        let instance_buffer = ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Instance Buffer"),
            size: (max_instances as u64) * std::mem::size_of::<InstanceData>() as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Camera uniform buffer
        let camera_buffer = ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Camera Buffer"),
            size: std::mem::size_of::<CameraUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Bind group layout
        let bind_group_layout = ctx.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bind Group Layout"),
            entries: &[
                // Camera uniform (used in both vertex and fragment shaders)
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // Instance storage buffer
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        // Bind group
        let bind_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: instance_buffer.as_entire_binding(),
                },
            ],
        });

        // Shadow bind group layout (group 1)
        let shadow_bind_group_layout = ctx.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Shadow Bind Group Layout"),
            entries: &[
                // Shadow uniforms (light view-projection)
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // Shadow map texture
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Depth,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                // Shadow sampler (comparison)
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Comparison),
                    count: None,
                },
            ],
        });

        // Shadow uniform buffer
        let shadow_uniform_buffer = ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Shadow Uniform Buffer"),
            size: std::mem::size_of::<ShadowUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Pipeline layout (includes shadow bind group)
        let pipeline_layout = ctx.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout, &shadow_bind_group_layout],
            push_constant_ranges: &[],
        });

        // Render pipeline
        let render_pipeline = ctx.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: HDR_FORMAT,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Self {
            render_pipeline,
            vertex_buffer,
            index_buffer,
            instance_buffer,
            camera_buffer,
            bind_group,
            shadow_bind_group_layout,
            shadow_uniform_buffer,
            shadow_bind_group: None,
            index_count,
            max_instances,
            half_extent,
        }
    }

    /// Upload instance data from positions, rotations, and colors
    pub fn upload_instances(
        &self,
        ctx: &GpuContext,
        positions: &[[f32; 3]],
        rotations: &[[f32; 4]],
        colors: &[[f32; 3]],
    ) {
        let instance_count = positions.len().min(self.max_instances as usize);
        let mut instances = Vec::with_capacity(instance_count);

        for i in 0..instance_count {
            instances.push(InstanceData {
                position: positions[i],
                _padding: 0.0,
                rotation: rotations[i],
                color: colors[i],
                _padding2: 0.0,
            });
        }

        ctx.queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&instances));
    }

    /// Update camera uniform
    pub fn update_camera(&self, ctx: &GpuContext, camera: &Camera) {
        let uniform = camera.uniform();
        ctx.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[uniform]));
    }

    /// Setup shadow bind group with shadow renderer
    pub fn setup_shadow(&mut self, ctx: &GpuContext, shadow_renderer: &ShadowRenderer) {
        let shadow_bind_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Cube Shadow Bind Group"),
            layout: &self.shadow_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: self.shadow_uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&shadow_renderer.shadow_view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&shadow_renderer.shadow_sampler),
                },
            ],
        });
        self.shadow_bind_group = Some(shadow_bind_group);
    }

    /// Update shadow uniforms (light view-projection matrix)
    pub fn update_shadow(&self, ctx: &GpuContext, light_view_proj: [[f32; 4]; 4]) {
        let uniform = ShadowUniform { light_view_proj };
        ctx.queue.write_buffer(&self.shadow_uniform_buffer, 0, bytemuck::cast_slice(&[uniform]));
    }

    /// Render instances to the HDR target
    pub fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        target: &OffscreenTarget,
        instance_count: u32,
    ) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Cube Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &target.hdr_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load, // Keep sky and ground
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &target.depth_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Load, // Keep ground depth
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);

        // Set shadow bind group if available
        if let Some(ref shadow_bind_group) = self.shadow_bind_group {
            render_pass.set_bind_group(1, shadow_bind_group, &[]);
        }

        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        // Single draw call for all instances!
        render_pass.draw_indexed(0..self.index_count, 0, 0..instance_count);
    }
}

/// Create cube vertex and index data with proper flat shading
/// Each face has 4 unique vertices with the same normal (24 total)
/// Winding is CCW when viewed from outside the cube
fn create_cube_geometry(half_extent: f32) -> (Vec<Vertex>, Vec<u16>) {
    let h = half_extent;

    let mut vertices = Vec::with_capacity(24);
    let mut indices = Vec::with_capacity(36);

    // Define each face explicitly with correct winding (CCW when viewed from outside)
    // Each face: 4 positions + 1 normal, vertices ordered for CCW winding

    // Front face (+Z normal) - viewed from +Z, CCW order
    let front_n = [0.0, 0.0, 1.0];
    vertices.push(Vertex { position: [-h, -h, h], normal: front_n }); // 0: bottom-left
    vertices.push(Vertex { position: [ h, -h, h], normal: front_n }); // 1: bottom-right
    vertices.push(Vertex { position: [ h,  h, h], normal: front_n }); // 2: top-right
    vertices.push(Vertex { position: [-h,  h, h], normal: front_n }); // 3: top-left

    // Back face (-Z normal) - viewed from -Z, CCW order
    let back_n = [0.0, 0.0, -1.0];
    vertices.push(Vertex { position: [ h, -h, -h], normal: back_n }); // 4: bottom-left (from -Z view)
    vertices.push(Vertex { position: [-h, -h, -h], normal: back_n }); // 5: bottom-right
    vertices.push(Vertex { position: [-h,  h, -h], normal: back_n }); // 6: top-right
    vertices.push(Vertex { position: [ h,  h, -h], normal: back_n }); // 7: top-left

    // Right face (+X normal) - viewed from +X, CCW order
    let right_n = [1.0, 0.0, 0.0];
    vertices.push(Vertex { position: [h, -h,  h], normal: right_n }); // 8: bottom-left
    vertices.push(Vertex { position: [h, -h, -h], normal: right_n }); // 9: bottom-right
    vertices.push(Vertex { position: [h,  h, -h], normal: right_n }); // 10: top-right
    vertices.push(Vertex { position: [h,  h,  h], normal: right_n }); // 11: top-left

    // Left face (-X normal) - viewed from -X, CCW order
    let left_n = [-1.0, 0.0, 0.0];
    vertices.push(Vertex { position: [-h, -h, -h], normal: left_n }); // 12: bottom-left
    vertices.push(Vertex { position: [-h, -h,  h], normal: left_n }); // 13: bottom-right
    vertices.push(Vertex { position: [-h,  h,  h], normal: left_n }); // 14: top-right
    vertices.push(Vertex { position: [-h,  h, -h], normal: left_n }); // 15: top-left

    // Top face (+Y normal) - viewed from +Y, CCW order
    let top_n = [0.0, 1.0, 0.0];
    vertices.push(Vertex { position: [-h, h,  h], normal: top_n }); // 16: front-left
    vertices.push(Vertex { position: [ h, h,  h], normal: top_n }); // 17: front-right
    vertices.push(Vertex { position: [ h, h, -h], normal: top_n }); // 18: back-right
    vertices.push(Vertex { position: [-h, h, -h], normal: top_n }); // 19: back-left

    // Bottom face (-Y normal) - viewed from -Y, CCW order
    let bottom_n = [0.0, -1.0, 0.0];
    vertices.push(Vertex { position: [-h, -h, -h], normal: bottom_n }); // 20: back-left
    vertices.push(Vertex { position: [ h, -h, -h], normal: bottom_n }); // 21: back-right
    vertices.push(Vertex { position: [ h, -h,  h], normal: bottom_n }); // 22: front-right
    vertices.push(Vertex { position: [-h, -h,  h], normal: bottom_n }); // 23: front-left

    // Generate indices for all 6 faces (2 triangles each, CCW winding)
    for face in 0..6 {
        let base = (face * 4) as u16;
        // First triangle: 0, 1, 2
        indices.push(base);
        indices.push(base + 1);
        indices.push(base + 2);
        // Second triangle: 0, 2, 3
        indices.push(base);
        indices.push(base + 2);
        indices.push(base + 3);
    }

    (vertices, indices)
}

// Required for buffer initialization
use wgpu::util::DeviceExt;
