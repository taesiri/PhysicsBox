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

    /// Get shape types (0 = cube, 1 = sphere)
    pub fn shape_types(&self) -> &[u8] {
        &self.storage.shape_types
    }

    /// Get radii/half-extents
    pub fn radii(&self) -> &[f32] {
        &self.storage.radii
    }

    /// Get cube data (positions and rotations for cubes only)
    pub fn cube_data(&self) -> (Vec<[f32; 3]>, Vec<[f32; 4]>) {
        let indices = self.storage.cube_indices();
        let positions: Vec<_> = indices.iter().map(|&i| self.storage.positions[i]).collect();
        let rotations: Vec<_> = indices.iter().map(|&i| self.storage.rotations[i]).collect();
        (positions, rotations)
    }

    /// Get sphere data (positions and radii for spheres only)
    pub fn sphere_data(&self) -> (Vec<[f32; 3]>, Vec<f32>) {
        let indices = self.storage.sphere_indices();
        let positions: Vec<_> = indices.iter().map(|&i| self.storage.positions[i]).collect();
        let radii: Vec<_> = indices.iter().map(|&i| self.storage.radii[i]).collect();
        (positions, radii)
    }
}
