//! GPU module - wgpu rendering with Metal backend

pub mod context;
pub mod render_target;
pub mod camera;
pub mod instance_renderer;
pub mod renderer;

pub use context::{GpuContext, GpuError};
pub use render_target::OffscreenTarget;
pub use camera::Camera;
pub use instance_renderer::InstanceRenderer;
pub use renderer::Renderer;
