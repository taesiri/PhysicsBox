//! Instance renderer for GPU-instanced cube rendering

use super::camera::{Camera, CameraUniform};
use super::context::GpuContext;
use super::render_target::OffscreenTarget;
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

/// Instance data (position + rotation)
#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct InstanceData {
    pub position: [f32; 3],
    pub _padding: f32,
    pub rotation: [f32; 4], // quaternion (x, y, z, w)
}

/// Instance renderer using GPU instancing
pub struct InstanceRenderer {
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
    camera_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
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

        // Pipeline layout
        let pipeline_layout = ctx.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
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
                    format: wgpu::TextureFormat::Rgba8UnormSrgb,
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
            index_count,
            max_instances,
            half_extent,
        }
    }

    /// Upload instance data from positions and rotations
    pub fn upload_instances(
        &self,
        ctx: &GpuContext,
        positions: &[[f32; 3]],
        rotations: &[[f32; 4]],
    ) {
        let instance_count = positions.len().min(self.max_instances as usize);
        let mut instances = Vec::with_capacity(instance_count);

        for i in 0..instance_count {
            instances.push(InstanceData {
                position: positions[i],
                _padding: 0.0,
                rotation: rotations[i],
            });
        }

        ctx.queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&instances));
    }

    /// Update camera uniform
    pub fn update_camera(&self, ctx: &GpuContext, camera: &Camera) {
        let uniform = camera.uniform();
        ctx.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[uniform]));
    }

    /// Render instances to the target
    pub fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        target: &OffscreenTarget,
        instance_count: u32,
    ) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Cube Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &target.view,
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
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        // Single draw call for all instances!
        render_pass.draw_indexed(0..self.index_count, 0, 0..instance_count);
    }
}

/// Create cube vertex and index data
fn create_cube_geometry(half_extent: f32) -> (Vec<Vertex>, Vec<u16>) {
    let h = half_extent;

    // 8 vertices of a cube
    let positions = [
        [-h, -h, -h], // 0: back bottom left
        [ h, -h, -h], // 1: back bottom right
        [ h,  h, -h], // 2: back top right
        [-h,  h, -h], // 3: back top left
        [-h, -h,  h], // 4: front bottom left
        [ h, -h,  h], // 5: front bottom right
        [ h,  h,  h], // 6: front top right
        [-h,  h,  h], // 7: front top left
    ];

    // Face normals
    let normals = [
        [ 0.0,  0.0, -1.0], // back
        [ 0.0,  0.0,  1.0], // front
        [-1.0,  0.0,  0.0], // left
        [ 1.0,  0.0,  0.0], // right
        [ 0.0, -1.0,  0.0], // bottom
        [ 0.0,  1.0,  0.0], // top
    ];

    // Faces: 6 faces, 4 vertices each (for flat shading with normals)
    let faces = [
        ([0, 1, 2, 3], 0), // back
        ([5, 4, 7, 6], 1), // front
        ([4, 0, 3, 7], 2), // left
        ([1, 5, 6, 2], 3), // right
        ([4, 5, 1, 0], 4), // bottom
        ([3, 2, 6, 7], 5), // top
    ];

    let mut vertices = Vec::with_capacity(24);
    let mut indices = Vec::with_capacity(36);

    for (face_indices, normal_idx) in faces.iter() {
        let base_vertex = vertices.len() as u16;

        for &vi in face_indices {
            vertices.push(Vertex {
                position: positions[vi],
                normal: normals[*normal_idx],
            });
        }

        // Two triangles per face
        indices.push(base_vertex);
        indices.push(base_vertex + 1);
        indices.push(base_vertex + 2);

        indices.push(base_vertex);
        indices.push(base_vertex + 2);
        indices.push(base_vertex + 3);
    }

    (vertices, indices)
}

// Required for buffer initialization
use wgpu::util::DeviceExt;
