//! Tonemapping post-process pass

use super::context::GpuContext;
use super::render_target::{OffscreenTarget, LDR_FORMAT};
use bytemuck::{Pod, Zeroable};

/// Tonemap parameters uniform
#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct TonemapParams {
    pub exposure: f32,
    pub _padding: [f32; 3],
}

impl Default for TonemapParams {
    fn default() -> Self {
        Self {
            exposure: 1.0,
            _padding: [0.0; 3],
        }
    }
}

/// Tonemapping renderer
pub struct TonemapRenderer {
    render_pipeline: wgpu::RenderPipeline,
    bind_group_layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
    params_buffer: wgpu::Buffer,
    params: TonemapParams,
}

impl TonemapRenderer {
    /// Create a new tonemap renderer
    pub fn new(ctx: &GpuContext) -> Self {
        // Create shader module
        let shader = ctx.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Tonemap Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/tonemap.wgsl").into()),
        });

        // Create sampler
        let sampler = ctx.device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Tonemap Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        // Create params buffer
        let params = TonemapParams::default();
        let params_buffer = ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Tonemap Params Buffer"),
            size: std::mem::size_of::<TonemapParams>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Bind group layout
        let bind_group_layout = ctx.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Tonemap Bind Group Layout"),
            entries: &[
                // HDR texture
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                // Sampler
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                // Params uniform
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        // Pipeline layout
        let pipeline_layout = ctx.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Tonemap Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        // Render pipeline
        let render_pipeline = ctx.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Tonemap Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],  // Fullscreen triangle generated in shader
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: LDR_FORMAT,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Self {
            render_pipeline,
            bind_group_layout,
            sampler,
            params_buffer,
            params,
        }
    }

    /// Set exposure value
    pub fn set_exposure(&mut self, exposure: f32) {
        self.params.exposure = exposure;
    }

    /// Render tonemap pass (HDR -> LDR)
    pub fn render(&self, ctx: &GpuContext, encoder: &mut wgpu::CommandEncoder, target: &OffscreenTarget) {
        // Update params buffer
        ctx.queue.write_buffer(&self.params_buffer, 0, bytemuck::cast_slice(&[self.params]));

        // Create bind group with current HDR texture
        let bind_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Tonemap Bind Group"),
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&target.hdr_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: self.params_buffer.as_entire_binding(),
                },
            ],
        });

        // Begin render pass to LDR target
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Tonemap Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &target.ldr_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &bind_group, &[]);
        render_pass.draw(0..3, 0..1);  // Fullscreen triangle
    }
}
