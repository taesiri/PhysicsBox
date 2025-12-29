//! Ground plane renderer with grid pattern and shadow support

use super::camera::{Camera, CameraUniform};
use super::context::GpuContext;
use super::render_target::{OffscreenTarget, HDR_FORMAT};
use super::shadow::ShadowRenderer;
use super::instance_renderer::ShadowUniform;
use bytemuck::{Pod, Zeroable};

/// Ground plane uniform data
#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct GroundUniform {
    pub ground_y: f32,
    pub ground_size: f32,
    pub grid_scale: f32,
    pub _padding: f32,
}

/// Renders a ground plane with grid pattern
pub struct GroundRenderer {
    pipeline: wgpu::RenderPipeline,
    camera_buffer: wgpu::Buffer,
    ground_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    // Shadow bindings
    shadow_bind_group_layout: wgpu::BindGroupLayout,
    shadow_uniform_buffer: wgpu::Buffer,
    shadow_bind_group: Option<wgpu::BindGroup>,
    ground_y: f32,
    ground_size: f32,
}

impl GroundRenderer {
    pub fn new(ctx: &GpuContext, ground_y: f32, ground_size: f32) -> Self {
        let shader = ctx.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Ground Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/ground.wgsl").into()),
        });

        // Camera buffer
        let camera_buffer = ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Ground Camera Buffer"),
            size: std::mem::size_of::<CameraUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Ground uniform buffer
        let ground_buffer = ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Ground Uniform Buffer"),
            size: std::mem::size_of::<GroundUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group_layout = ctx.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Ground Bind Group Layout"),
            entries: &[
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
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let bind_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Ground Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: ground_buffer.as_entire_binding(),
                },
            ],
        });

        // Shadow bind group layout (group 1)
        let shadow_bind_group_layout = ctx.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Ground Shadow Bind Group Layout"),
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
            label: Some("Ground Shadow Uniform Buffer"),
            size: std::mem::size_of::<ShadowUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Pipeline layout (includes shadow bind group)
        let pipeline_layout = ctx.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Ground Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout, &shadow_bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = ctx.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Ground Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: HDR_FORMAT,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                cull_mode: None, // Render both sides
                ..Default::default()
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
            pipeline,
            camera_buffer,
            ground_buffer,
            bind_group,
            shadow_bind_group_layout,
            shadow_uniform_buffer,
            shadow_bind_group: None,
            ground_y,
            ground_size,
        }
    }

    pub fn update_camera(&self, ctx: &GpuContext, camera: &Camera) {
        let uniform = camera.uniform();
        ctx.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[uniform]));
    }

    pub fn update_ground(&self, ctx: &GpuContext, ground_y: f32, ground_size: f32, grid_scale: f32) {
        let uniform = GroundUniform {
            ground_y,
            ground_size,
            grid_scale,
            _padding: 0.0,
        };
        ctx.queue.write_buffer(&self.ground_buffer, 0, bytemuck::cast_slice(&[uniform]));
    }

    /// Setup shadow bind group with shadow renderer
    pub fn setup_shadow(&mut self, ctx: &GpuContext, shadow_renderer: &ShadowRenderer) {
        let shadow_bind_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Ground Shadow Bind Group"),
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

    pub fn render(&self, encoder: &mut wgpu::CommandEncoder, target: &OffscreenTarget) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Ground Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &target.hdr_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load, // Keep sky background
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &target.depth_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);

        // Set shadow bind group if available
        if let Some(ref shadow_bind_group) = self.shadow_bind_group {
            render_pass.set_bind_group(1, shadow_bind_group, &[]);
        }

        render_pass.draw(0..6, 0..1); // Two triangles for quad
    }

    pub fn ground_y(&self) -> f32 {
        self.ground_y
    }

    pub fn ground_size(&self) -> f32 {
        self.ground_size
    }
}
