//! Offscreen render target for headless rendering with HDR support

use super::context::GpuContext;

/// HDR render target format
pub const HDR_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
/// LDR output format (for file output)
pub const LDR_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;

/// Offscreen render target with HDR rendering and LDR output
pub struct OffscreenTarget {
    /// HDR render texture (scene renders here)
    pub hdr_texture: wgpu::Texture,
    /// HDR texture view for rendering
    pub hdr_view: wgpu::TextureView,
    /// LDR output texture (tonemapped result)
    pub ldr_texture: wgpu::Texture,
    /// LDR texture view
    pub ldr_view: wgpu::TextureView,
    /// Depth texture
    pub depth_texture: wgpu::Texture,
    /// Depth texture view
    pub depth_view: wgpu::TextureView,
    /// Staging buffer for CPU readback
    pub output_buffer: wgpu::Buffer,
    /// Width in pixels
    pub width: u32,
    /// Height in pixels
    pub height: u32,
    /// Padded bytes per row (aligned to 256)
    pub padded_bytes_per_row: u32,
}

impl OffscreenTarget {
    /// Create a 4K render target (3840x2160)
    pub fn new_4k(ctx: &GpuContext) -> Self {
        Self::new(ctx, 3840, 2160)
    }

    /// Create a 1080p render target (1920x1080)
    pub fn new_1080p(ctx: &GpuContext) -> Self {
        Self::new(ctx, 1920, 1080)
    }

    /// Create a render target with custom dimensions
    pub fn new(ctx: &GpuContext, width: u32, height: u32) -> Self {
        // Calculate padded bytes per row (must be multiple of 256)
        let bytes_per_pixel = 4; // RGBA8 for LDR output
        let unpadded_bytes_per_row = width * bytes_per_pixel;
        let padded_bytes_per_row = (unpadded_bytes_per_row + 255) & !255;

        // Create HDR render texture (scene renders here)
        let hdr_texture = ctx.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("HDR Render Target"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: HDR_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                 | wgpu::TextureUsages::TEXTURE_BINDING,  // For tonemap sampling
            view_formats: &[],
        });

        let hdr_view = hdr_texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Create LDR output texture (tonemapped result, for file output)
        let ldr_texture = ctx.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("LDR Output Target"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: LDR_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });

        let ldr_view = ldr_texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Create depth texture
        let depth_texture = ctx.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Create output buffer for CPU readback (reads from LDR texture)
        let buffer_size = (padded_bytes_per_row * height) as u64;
        let output_buffer = ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Output Buffer"),
            size: buffer_size,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        Self {
            hdr_texture,
            hdr_view,
            ldr_texture,
            ldr_view,
            depth_texture,
            depth_view,
            output_buffer,
            width,
            height,
            padded_bytes_per_row,
        }
    }

    /// Copy LDR texture to staging buffer (call after tonemapping)
    pub fn copy_to_buffer(&self, encoder: &mut wgpu::CommandEncoder) {
        encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                texture: &self.ldr_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::ImageCopyBuffer {
                buffer: &self.output_buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(self.padded_bytes_per_row),
                    rows_per_image: Some(self.height),
                },
            },
            wgpu::Extent3d {
                width: self.width,
                height: self.height,
                depth_or_array_layers: 1,
            },
        );
    }

    /// Read pixels from staging buffer (blocking)
    pub fn read_pixels(&self, ctx: &GpuContext) -> Vec<u8> {
        let buffer_slice = self.output_buffer.slice(..);

        // Map buffer
        let (tx, rx) = std::sync::mpsc::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            tx.send(result).unwrap();
        });

        // Wait for mapping
        ctx.device.poll(wgpu::Maintain::Wait);
        rx.recv().unwrap().unwrap();

        // Read data
        let data = buffer_slice.get_mapped_range();

        // Remove padding and create output
        let bytes_per_pixel = 4;
        let unpadded_bytes_per_row = self.width * bytes_per_pixel;
        let mut output = Vec::with_capacity((unpadded_bytes_per_row * self.height) as usize);

        for y in 0..self.height {
            let start = (y * self.padded_bytes_per_row) as usize;
            let end = start + unpadded_bytes_per_row as usize;
            output.extend_from_slice(&data[start..end]);
        }

        // Unmap buffer
        drop(data);
        self.output_buffer.unmap();

        output
    }
}
