//! Shadow map renderer for directional light shadows

use super::context::GpuContext;
use super::instance_renderer::InstanceData;
use super::sphere_renderer::SphereInstanceData;
use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

/// Shadow map resolution
pub const SHADOW_MAP_SIZE: u32 = 2048;

/// Light camera uniform for shadow pass
#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct LightCameraUniform {
    pub view_proj: [[f32; 4]; 4],
}

/// Vertex data for shadow geometry
#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
struct ShadowVertex {
    position: [f32; 3],
    normal: [f32; 3],
}

impl ShadowVertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x3,
        1 => Float32x3,
    ];

    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<ShadowVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

/// Shadow map renderer
pub struct ShadowRenderer {
    // Shadow map texture
    pub shadow_texture: wgpu::Texture,
    pub shadow_view: wgpu::TextureView,
    pub shadow_sampler: wgpu::Sampler,

    // Cube shadow pass
    cube_pipeline: wgpu::RenderPipeline,
    cube_vertex_buffer: wgpu::Buffer,
    cube_index_buffer: wgpu::Buffer,
    cube_index_count: u32,
    cube_instance_buffer: wgpu::Buffer,
    cube_bind_group: wgpu::BindGroup,

    // Sphere shadow pass
    sphere_pipeline: wgpu::RenderPipeline,
    sphere_vertex_buffer: wgpu::Buffer,
    sphere_index_buffer: wgpu::Buffer,
    sphere_index_count: u32,
    sphere_instance_buffer: wgpu::Buffer,
    sphere_bind_group: wgpu::BindGroup,

    // Shared light camera buffer
    light_camera_buffer: wgpu::Buffer,

    // Light direction (normalized)
    light_dir: [f32; 3],

    // Shadow frustum size
    frustum_size: f32,

    max_instances: u32,
    half_extent: f32,
}

impl ShadowRenderer {
    pub fn new(ctx: &GpuContext, max_instances: u32, half_extent: f32) -> Self {
        // Create shadow map texture
        let shadow_texture = ctx.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Shadow Map"),
            size: wgpu::Extent3d {
                width: SHADOW_MAP_SIZE,
                height: SHADOW_MAP_SIZE,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let shadow_view = shadow_texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Comparison sampler for shadow mapping
        let shadow_sampler = ctx.device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Shadow Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            compare: Some(wgpu::CompareFunction::LessEqual),
            ..Default::default()
        });

        // Light camera buffer
        let light_camera_buffer = ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Light Camera Buffer"),
            size: std::mem::size_of::<LightCameraUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create shader module
        let shader = ctx.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shadow Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/shadow_depth.wgsl").into()),
        });

        // === Cube shadow pipeline ===
        let (cube_vertices, cube_indices) = create_cube_geometry(half_extent);
        let cube_index_count = cube_indices.len() as u32;

        let cube_vertex_buffer = ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Shadow Cube Vertex Buffer"),
            contents: bytemuck::cast_slice(&cube_vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let cube_index_buffer = ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Shadow Cube Index Buffer"),
            contents: bytemuck::cast_slice(&cube_indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let cube_instance_buffer = ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Shadow Cube Instance Buffer"),
            size: (max_instances as u64) * std::mem::size_of::<InstanceData>() as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Sphere instance buffer (for binding, using dummy for now)
        let sphere_instance_buffer = ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Shadow Sphere Instance Buffer"),
            size: (max_instances as u64) * std::mem::size_of::<SphereInstanceData>() as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Bind group layout for shadow pass
        let bind_group_layout = ctx.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Shadow Bind Group Layout"),
            entries: &[
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
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
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

        let cube_bind_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Shadow Cube Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: light_camera_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: cube_instance_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: sphere_instance_buffer.as_entire_binding(),
                },
            ],
        });

        let pipeline_layout = ctx.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Shadow Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let cube_pipeline = ctx.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Shadow Cube Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_cube"),
                buffers: &[ShadowVertex::desc()],
                compilation_options: Default::default(),
            },
            fragment: None, // Depth-only
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState {
                    constant: 2,
                    slope_scale: 2.0,
                    clamp: 0.0,
                },
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        // === Sphere shadow pipeline ===
        let (sphere_vertices, sphere_indices) = create_sphere_geometry(16, 12);
        let sphere_index_count = sphere_indices.len() as u32;

        let sphere_vertex_buffer = ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Shadow Sphere Vertex Buffer"),
            contents: bytemuck::cast_slice(&sphere_vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let sphere_index_buffer = ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Shadow Sphere Index Buffer"),
            contents: bytemuck::cast_slice(&sphere_indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let sphere_bind_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Shadow Sphere Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: light_camera_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: cube_instance_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: sphere_instance_buffer.as_entire_binding(),
                },
            ],
        });

        let sphere_pipeline = ctx.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Shadow Sphere Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_sphere"),
                buffers: &[ShadowVertex::desc()],
                compilation_options: Default::default(),
            },
            fragment: None, // Depth-only
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState {
                    constant: 2,
                    slope_scale: 2.0,
                    clamp: 0.0,
                },
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        // Default light direction (same as key light in shaders)
        let light_dir = normalize([-0.5, 0.9, 0.6]);

        Self {
            shadow_texture,
            shadow_view,
            shadow_sampler,
            cube_pipeline,
            cube_vertex_buffer,
            cube_index_buffer,
            cube_index_count,
            cube_instance_buffer,
            cube_bind_group,
            sphere_pipeline,
            sphere_vertex_buffer,
            sphere_index_buffer,
            sphere_index_count,
            sphere_instance_buffer,
            sphere_bind_group,
            light_camera_buffer,
            light_dir,
            frustum_size: 100.0,
            max_instances,
            half_extent,
        }
    }

    /// Set the light direction (will be normalized)
    pub fn set_light_direction(&mut self, dir: [f32; 3]) {
        self.light_dir = normalize(dir);
    }

    /// Set shadow frustum size (how large an area the shadow covers)
    pub fn set_frustum_size(&mut self, size: f32) {
        self.frustum_size = size;
    }

    /// Upload cube instances for shadow rendering
    pub fn upload_cube_instances(
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

        ctx.queue.write_buffer(&self.cube_instance_buffer, 0, bytemuck::cast_slice(&instances));
    }

    /// Upload sphere instances for shadow rendering
    pub fn upload_sphere_instances(
        &self,
        ctx: &GpuContext,
        positions: &[[f32; 3]],
        radii: &[f32],
        colors: &[[f32; 3]],
    ) {
        let instance_count = positions.len().min(self.max_instances as usize);
        let mut instances = Vec::with_capacity(instance_count);

        for i in 0..instance_count {
            instances.push(SphereInstanceData {
                position: positions[i],
                radius: radii[i],
                rotation: [0.0, 0.0, 0.0, 1.0],
                color: colors[i],
                _padding: 0.0,
            });
        }

        ctx.queue.write_buffer(&self.sphere_instance_buffer, 0, bytemuck::cast_slice(&instances));
    }

    /// Update light camera for shadow pass (orthographic projection from light direction)
    pub fn update_light_camera(&self, ctx: &GpuContext, scene_center: [f32; 3]) {
        let view_proj = self.compute_light_view_proj(scene_center);
        let uniform = LightCameraUniform { view_proj };
        ctx.queue.write_buffer(&self.light_camera_buffer, 0, bytemuck::cast_slice(&[uniform]));
    }

    /// Compute light view-projection matrix
    pub fn compute_light_view_proj(&self, scene_center: [f32; 3]) -> [[f32; 4]; 4] {
        // Position light far above scene, looking at center
        let light_distance = self.frustum_size * 2.0;
        let light_pos = [
            scene_center[0] + self.light_dir[0] * light_distance,
            scene_center[1] + self.light_dir[1] * light_distance,
            scene_center[2] + self.light_dir[2] * light_distance,
        ];

        // View matrix (look at scene center)
        let view = look_at(light_pos, scene_center, [0.0, 1.0, 0.0]);

        // Orthographic projection
        let half = self.frustum_size;
        let near = 0.1;
        let far = light_distance * 2.0;
        let proj = ortho(-half, half, -half, half, near, far);

        // Combine
        mat4_mul(&proj, &view)
    }

    /// Get the light view-projection matrix for use in main shaders
    pub fn get_light_view_proj(&self, scene_center: [f32; 3]) -> [[f32; 4]; 4] {
        self.compute_light_view_proj(scene_center)
    }

    /// Render shadow map
    pub fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        cube_count: u32,
        sphere_count: u32,
    ) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Shadow Render Pass"),
            color_attachments: &[],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.shadow_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        // Render cubes to shadow map
        if cube_count > 0 {
            render_pass.set_pipeline(&self.cube_pipeline);
            render_pass.set_bind_group(0, &self.cube_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.cube_vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.cube_index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.cube_index_count, 0, 0..cube_count);
        }

        // Render spheres to shadow map
        if sphere_count > 0 {
            render_pass.set_pipeline(&self.sphere_pipeline);
            render_pass.set_bind_group(0, &self.sphere_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.sphere_vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.sphere_index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.sphere_index_count, 0, 0..sphere_count);
        }
    }
}

// === Helper functions ===

fn normalize(v: [f32; 3]) -> [f32; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    [v[0] / len, v[1] / len, v[2] / len]
}

fn cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn look_at(eye: [f32; 3], target: [f32; 3], up: [f32; 3]) -> [[f32; 4]; 4] {
    let f = normalize([target[0] - eye[0], target[1] - eye[1], target[2] - eye[2]]);
    let s = normalize(cross(f, up));
    let u = cross(s, f);

    [
        [s[0], u[0], -f[0], 0.0],
        [s[1], u[1], -f[1], 0.0],
        [s[2], u[2], -f[2], 0.0],
        [-dot(s, eye), -dot(u, eye), dot(f, eye), 1.0],
    ]
}

fn ortho(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> [[f32; 4]; 4] {
    let rml = right - left;
    let tmb = top - bottom;
    let fmn = far - near;

    [
        [2.0 / rml, 0.0, 0.0, 0.0],
        [0.0, 2.0 / tmb, 0.0, 0.0],
        [0.0, 0.0, -1.0 / fmn, 0.0],
        [-(right + left) / rml, -(top + bottom) / tmb, -near / fmn, 1.0],
    ]
}

fn mat4_mul(a: &[[f32; 4]; 4], b: &[[f32; 4]; 4]) -> [[f32; 4]; 4] {
    let mut result = [[0.0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                result[i][j] += a[k][j] * b[i][k];
            }
        }
    }
    result
}

/// Create cube geometry (same as main renderer)
fn create_cube_geometry(half_extent: f32) -> (Vec<ShadowVertex>, Vec<u16>) {
    let h = half_extent;
    let mut vertices = Vec::with_capacity(24);
    let mut indices = Vec::with_capacity(36);

    // Front face (+Z)
    let front_n = [0.0, 0.0, 1.0];
    vertices.push(ShadowVertex { position: [-h, -h, h], normal: front_n });
    vertices.push(ShadowVertex { position: [ h, -h, h], normal: front_n });
    vertices.push(ShadowVertex { position: [ h,  h, h], normal: front_n });
    vertices.push(ShadowVertex { position: [-h,  h, h], normal: front_n });

    // Back face (-Z)
    let back_n = [0.0, 0.0, -1.0];
    vertices.push(ShadowVertex { position: [ h, -h, -h], normal: back_n });
    vertices.push(ShadowVertex { position: [-h, -h, -h], normal: back_n });
    vertices.push(ShadowVertex { position: [-h,  h, -h], normal: back_n });
    vertices.push(ShadowVertex { position: [ h,  h, -h], normal: back_n });

    // Right face (+X)
    let right_n = [1.0, 0.0, 0.0];
    vertices.push(ShadowVertex { position: [h, -h,  h], normal: right_n });
    vertices.push(ShadowVertex { position: [h, -h, -h], normal: right_n });
    vertices.push(ShadowVertex { position: [h,  h, -h], normal: right_n });
    vertices.push(ShadowVertex { position: [h,  h,  h], normal: right_n });

    // Left face (-X)
    let left_n = [-1.0, 0.0, 0.0];
    vertices.push(ShadowVertex { position: [-h, -h, -h], normal: left_n });
    vertices.push(ShadowVertex { position: [-h, -h,  h], normal: left_n });
    vertices.push(ShadowVertex { position: [-h,  h,  h], normal: left_n });
    vertices.push(ShadowVertex { position: [-h,  h, -h], normal: left_n });

    // Top face (+Y)
    let top_n = [0.0, 1.0, 0.0];
    vertices.push(ShadowVertex { position: [-h, h,  h], normal: top_n });
    vertices.push(ShadowVertex { position: [ h, h,  h], normal: top_n });
    vertices.push(ShadowVertex { position: [ h, h, -h], normal: top_n });
    vertices.push(ShadowVertex { position: [-h, h, -h], normal: top_n });

    // Bottom face (-Y)
    let bottom_n = [0.0, -1.0, 0.0];
    vertices.push(ShadowVertex { position: [-h, -h, -h], normal: bottom_n });
    vertices.push(ShadowVertex { position: [ h, -h, -h], normal: bottom_n });
    vertices.push(ShadowVertex { position: [ h, -h,  h], normal: bottom_n });
    vertices.push(ShadowVertex { position: [-h, -h,  h], normal: bottom_n });

    for face in 0..6 {
        let base = (face * 4) as u16;
        indices.push(base);
        indices.push(base + 1);
        indices.push(base + 2);
        indices.push(base);
        indices.push(base + 2);
        indices.push(base + 3);
    }

    (vertices, indices)
}

/// Create sphere geometry (same as main renderer)
fn create_sphere_geometry(segments: u32, rings: u32) -> (Vec<ShadowVertex>, Vec<u16>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for ring in 0..=rings {
        let phi = std::f32::consts::PI * ring as f32 / rings as f32;
        let sin_phi = phi.sin();
        let cos_phi = phi.cos();

        for seg in 0..=segments {
            let theta = 2.0 * std::f32::consts::PI * seg as f32 / segments as f32;
            let sin_theta = theta.sin();
            let cos_theta = theta.cos();

            let x = sin_phi * cos_theta;
            let y = cos_phi;
            let z = sin_phi * sin_theta;

            vertices.push(ShadowVertex {
                position: [x, y, z],
                normal: [x, y, z],
            });
        }
    }

    for ring in 0..rings {
        for seg in 0..segments {
            let current = ring * (segments + 1) + seg;
            let next = current + segments + 1;

            indices.push(current as u16);
            indices.push(next as u16);
            indices.push((current + 1) as u16);

            indices.push((current + 1) as u16);
            indices.push(next as u16);
            indices.push((next + 1) as u16);
        }
    }

    (vertices, indices)
}
