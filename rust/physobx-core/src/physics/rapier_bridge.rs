//! Bridge between SOA storage and Rapier physics engine

use rapier3d::prelude::*;
use super::storage::RigidBodyStorage;
use crate::scene::builder::{SceneBuilder, RigidBodyConfig, ShapeType};

/// Bridge for syncing with Rapier physics
pub struct RapierBridge {
    /// Rapier rigid body set
    pub rigid_body_set: RigidBodySet,
    /// Rapier collider set
    pub collider_set: ColliderSet,
    /// Physics pipeline
    physics_pipeline: PhysicsPipeline,
    /// Island manager
    island_manager: IslandManager,
    /// Broad phase
    broad_phase: DefaultBroadPhase,
    /// Narrow phase
    narrow_phase: NarrowPhase,
    /// Impulse joints
    impulse_joint_set: ImpulseJointSet,
    /// Multibody joints
    multibody_joint_set: MultibodyJointSet,
    /// CCD solver
    ccd_solver: CCDSolver,
    /// Query pipeline for raycasts
    query_pipeline: QueryPipeline,
    /// Gravity vector
    gravity: Vector<Real>,
    /// Integration parameters
    integration_parameters: IntegrationParameters,
    /// Mapping from SOA index to Rapier handle
    body_handles: Vec<RigidBodyHandle>,
    /// Mapping from SOA index to Collider handle
    collider_handles: Vec<ColliderHandle>,
}

impl Default for RapierBridge {
    fn default() -> Self {
        Self::new()
    }
}

impl RapierBridge {
    /// Create a new Rapier bridge
    pub fn new() -> Self {
        Self {
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: DefaultBroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            query_pipeline: QueryPipeline::new(),
            gravity: vector![0.0, -9.81, 0.0],
            integration_parameters: IntegrationParameters::default(),
            body_handles: Vec::new(),
            collider_handles: Vec::new(),
        }
    }

    /// Build physics world from scene
    pub fn build_from_scene(&mut self, scene: &SceneBuilder, storage: &mut RigidBodyStorage) {
        // Clear existing
        self.rigid_body_set = RigidBodySet::new();
        self.collider_set = ColliderSet::new();
        self.body_handles.clear();
        self.collider_handles.clear();
        storage.clear();

        // Add ground if specified
        if let Some(ground_y) = scene.ground_y {
            let ground = RigidBodyBuilder::fixed()
                .translation(vector![0.0, ground_y, 0.0])
                .build();
            let ground_handle = self.rigid_body_set.insert(ground);

            let ground_collider = ColliderBuilder::cuboid(
                scene.ground_size,
                0.1,
                scene.ground_size,
            )
            .restitution(0.3)
            .friction(0.5)
            .build();
            self.collider_set.insert_with_parent(ground_collider, ground_handle, &mut self.rigid_body_set);
        }

        // Add dynamic bodies
        for config in &scene.bodies {
            self.add_body(config, storage);
        }
    }

    /// Add a single rigid body
    fn add_body(&mut self, config: &RigidBodyConfig, storage: &mut RigidBodyStorage) {
        // Create Rapier body with optional initial velocity
        let mut body_builder = RigidBodyBuilder::dynamic()
            .translation(vector![config.position[0], config.position[1], config.position[2]])
            .rotation(vector![
                config.rotation[0],
                config.rotation[1],
                config.rotation[2],
            ]);

        // Set initial velocity if non-zero
        if config.velocity != [0.0, 0.0, 0.0] {
            body_builder = body_builder.linvel(vector![
                config.velocity[0],
                config.velocity[1],
                config.velocity[2],
            ]);
        }

        let body = body_builder.build();
        let body_handle = self.rigid_body_set.insert(body);

        // Create collider based on shape type
        let collider = match config.shape {
            ShapeType::Cube => {
                let volume = 8.0 * config.half_extents[0] * config.half_extents[1] * config.half_extents[2];
                ColliderBuilder::cuboid(
                    config.half_extents[0],
                    config.half_extents[1],
                    config.half_extents[2],
                )
                .restitution(config.restitution)
                .friction(config.friction)
                .density(config.mass / volume)
                .build()
            }
            ShapeType::Sphere => {
                let volume = (4.0 / 3.0) * std::f32::consts::PI * config.radius.powi(3);
                ColliderBuilder::ball(config.radius)
                    .restitution(config.restitution)
                    .friction(config.friction)
                    .density(config.mass / volume)
                    .build()
            }
        };

        let collider_handle = self.collider_set.insert_with_parent(
            collider,
            body_handle,
            &mut self.rigid_body_set,
        );

        // Add to SOA storage with shape info
        storage.push_with_shape(config.position, config.rotation, config.mass, config.shape, config.radius, config.half_extents[0], config.color);

        // Store handles
        self.body_handles.push(body_handle);
        self.collider_handles.push(collider_handle);
    }

    /// Step the physics simulation
    pub fn step(&mut self, dt: f32) {
        self.integration_parameters.dt = dt;

        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            Some(&mut self.query_pipeline),
            &(),
            &(),
        );
    }

    /// Sync Rapier state back to SOA storage
    pub fn sync_to_storage(&self, storage: &mut RigidBodyStorage) {
        for (i, handle) in self.body_handles.iter().enumerate() {
            if let Some(body) = self.rigid_body_set.get(*handle) {
                let pos = body.translation();
                let rot = body.rotation();
                let lin_vel = body.linvel();
                let ang_vel = body.angvel();

                storage.positions[i] = [pos.x, pos.y, pos.z];
                storage.rotations[i] = [rot.i, rot.j, rot.k, rot.w];
                storage.linear_velocities[i] = [lin_vel.x, lin_vel.y, lin_vel.z];
                storage.angular_velocities[i] = [ang_vel.x, ang_vel.y, ang_vel.z];
            }
        }
    }

    /// Get number of dynamic bodies
    pub fn body_count(&self) -> usize {
        self.body_handles.len()
    }
}
