//! Physobx Core - Physics simulation and GPU rendering library
//!
//! This crate provides the core functionality for the Physobx physics sandbox:
//! - SOA (Structure of Arrays) rigid body storage
//! - Rapier physics integration
//! - wgpu-based GPU rendering with Metal backend
//! - Headless offscreen rendering

pub mod physics;
pub mod scene;
pub mod gpu;
pub mod simulator;

pub use physics::{RigidBodyStorage, RapierBridge};
pub use scene::SceneBuilder;
pub use simulator::Simulator;
pub use gpu::{GpuContext, GpuError, OffscreenTarget, Camera, InstanceRenderer};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Get the library version
pub fn version() -> &'static str {
    VERSION
}
