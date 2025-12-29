//! Physics module - SOA storage and Rapier integration

pub mod storage;
pub mod rapier_bridge;

pub use storage::RigidBodyStorage;
pub use rapier_bridge::RapierBridge;
