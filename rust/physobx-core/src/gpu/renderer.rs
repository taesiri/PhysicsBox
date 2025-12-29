//! Complete renderer combining all GPU components

use super::{GpuContext, GpuError, OffscreenTarget, Camera, InstanceRenderer, SkyRenderer, GroundRenderer};

/// Complete renderer for physics simulation
pub struct Renderer {
    pub ctx: GpuContext,
    pub target: OffscreenTarget,
    pub sky_renderer: SkyRenderer,
    pub ground_renderer: GroundRenderer,
    pub instance_renderer: InstanceRenderer,
    pub camera: Camera,
    ground_y: f32,
    ground_size: f32,
}

impl Renderer {
    /// Create a new renderer with specified dimensions
    pub fn new(
        width: u32,
        height: u32,
        max_instances: u32,
        half_extent: f32,
        ground_y: f32,
        ground_size: f32,
    ) -> Result<Self, GpuError> {
        let ctx = GpuContext::new_headless()?;
        let target = OffscreenTarget::new(&ctx, width, height);
        let sky_renderer = SkyRenderer::new(&ctx);
        let ground_renderer = GroundRenderer::new(&ctx, ground_y, ground_size);
        let instance_renderer = InstanceRenderer::new(&ctx, max_instances, half_extent);

        let mut camera = Camera::default();
        camera.set_aspect(width, height);

        Ok(Self {
            ctx,
            target,
            sky_renderer,
            ground_renderer,
            instance_renderer,
            camera,
            ground_y,
            ground_size,
        })
    }

    /// Create a 1080p renderer
    pub fn new_1080p(max_instances: u32, half_extent: f32, ground_y: f32, ground_size: f32) -> Result<Self, GpuError> {
        Self::new(1920, 1080, max_instances, half_extent, ground_y, ground_size)
    }

    /// Create a 4K renderer
    pub fn new_4k(max_instances: u32, half_extent: f32, ground_y: f32, ground_size: f32) -> Result<Self, GpuError> {
        Self::new(3840, 2160, max_instances, half_extent, ground_y, ground_size)
    }

    /// Set camera position and target
    pub fn set_camera(&mut self, eye: [f32; 3], target: [f32; 3]) {
        self.camera.eye = eye.into();
        self.camera.target = target.into();
    }

    /// Render a frame and return RGBA pixel data
    pub fn render_frame(&self, positions: &[[f32; 3]], rotations: &[[f32; 4]]) -> Vec<u8> {
        let instance_count = positions.len() as u32;

        // Upload instance data
        self.instance_renderer.upload_instances(&self.ctx, positions, rotations);

        // Update camera for all renderers
        self.instance_renderer.update_camera(&self.ctx, &self.camera);
        self.ground_renderer.update_camera(&self.ctx, &self.camera);
        self.ground_renderer.update_ground(&self.ctx, self.ground_y, self.ground_size, 5.0);

        // Create command encoder
        let mut encoder = self.ctx.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        // Render order: sky -> ground -> cubes
        self.sky_renderer.render(&mut encoder, &self.target);
        self.ground_renderer.render(&mut encoder, &self.target);
        self.instance_renderer.render(&mut encoder, &self.target, instance_count);

        // Copy to staging buffer
        self.target.copy_to_buffer(&mut encoder);

        // Submit commands
        self.ctx.queue.submit(std::iter::once(encoder.finish()));

        // Read pixels
        self.target.read_pixels(&self.ctx)
    }

    /// Save frame as PNG
    pub fn save_png(&self, positions: &[[f32; 3]], rotations: &[[f32; 4]], path: &str) -> Result<(), image::ImageError> {
        let pixels = self.render_frame(positions, rotations);

        image::save_buffer(
            path,
            &pixels,
            self.target.width,
            self.target.height,
            image::ColorType::Rgba8,
        )
    }

    /// Get dimensions
    pub fn dimensions(&self) -> (u32, u32) {
        (self.target.width, self.target.height)
    }
}
