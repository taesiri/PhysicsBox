//! Simulator - Main simulation orchestration

use crate::physics::{RigidBodyStorage, RapierBridge};
use crate::scene::SceneBuilder;

/// Main physics simulator
pub struct Simulator {
    /// SOA storage for rigid body data
    pub storage: RigidBodyStorage,
    /// Rapier physics bridge
    pub physics: RapierBridge,
    /// Current simulation time
    pub time: f32,
}

impl Simulator {
    /// Create a new simulator from a scene
    pub fn new(scene: &SceneBuilder) -> Self {
        let mut storage = RigidBodyStorage::with_capacity(scene.bodies.len());
        let mut physics = RapierBridge::new();
        physics.build_from_scene(scene, &mut storage);

        Self {
            storage,
            physics,
            time: 0.0,
        }
    }

    /// Step the simulation forward by dt seconds
    pub fn step(&mut self, dt: f32) {
        self.physics.step(dt);
        self.physics.sync_to_storage(&mut self.storage);
        self.time += dt;
    }

    /// Get number of bodies
    pub fn body_count(&self) -> usize {
        self.storage.len()
    }

    /// Get positions slice
    pub fn positions(&self) -> &[[f32; 3]] {
        &self.storage.positions
    }

    /// Get rotations slice
    pub fn rotations(&self) -> &[[f32; 4]] {
        &self.storage.rotations
    }
}
