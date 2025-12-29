//! SOA (Structure of Arrays) storage for rigid body data
//!
//! This provides cache-friendly, SIMD-optimized storage for physics state.

use crate::scene::builder::ShapeType;

/// SOA storage for rigid body state
#[derive(Debug, Default)]
pub struct RigidBodyStorage {
    /// Position vectors (x, y, z)
    pub positions: Vec<[f32; 3]>,
    /// Rotation quaternions (x, y, z, w)
    pub rotations: Vec<[f32; 4]>,
    /// Linear velocities
    pub linear_velocities: Vec<[f32; 3]>,
    /// Angular velocities
    pub angular_velocities: Vec<[f32; 3]>,
    /// Masses
    pub masses: Vec<f32>,
    /// Shape types (0 = cube, 1 = sphere)
    pub shape_types: Vec<u8>,
    /// Radii (for spheres) or half-extents (for cubes)
    pub radii: Vec<f32>,
    /// Colors (RGB)
    pub colors: Vec<[f32; 3]>,
}

impl RigidBodyStorage {
    /// Create storage with pre-allocated capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            positions: Vec::with_capacity(capacity),
            rotations: Vec::with_capacity(capacity),
            linear_velocities: Vec::with_capacity(capacity),
            angular_velocities: Vec::with_capacity(capacity),
            masses: Vec::with_capacity(capacity),
            shape_types: Vec::with_capacity(capacity),
            radii: Vec::with_capacity(capacity),
            colors: Vec::with_capacity(capacity),
        }
    }

    /// Number of bodies stored
    pub fn len(&self) -> usize {
        self.positions.len()
    }

    /// Check if storage is empty
    pub fn is_empty(&self) -> bool {
        self.positions.is_empty()
    }

    /// Add a new rigid body (cube by default)
    pub fn push(
        &mut self,
        position: [f32; 3],
        rotation: [f32; 4],
        mass: f32,
    ) -> usize {
        self.push_with_shape(position, rotation, mass, ShapeType::Cube, 0.5, 0.5, [0.82, 0.32, 0.12])
    }

    /// Add a new rigid body with shape info
    pub fn push_with_shape(
        &mut self,
        position: [f32; 3],
        rotation: [f32; 4],
        mass: f32,
        shape: ShapeType,
        radius: f32,
        half_extent: f32,
        color: [f32; 3],
    ) -> usize {
        let index = self.positions.len();
        self.positions.push(position);
        self.rotations.push(rotation);
        self.linear_velocities.push([0.0, 0.0, 0.0]);
        self.angular_velocities.push([0.0, 0.0, 0.0]);
        self.masses.push(mass);
        self.shape_types.push(match shape {
            ShapeType::Cube => 0,
            ShapeType::Sphere => 1,
        });
        self.radii.push(match shape {
            ShapeType::Sphere => radius,
            ShapeType::Cube => half_extent,
        });
        self.colors.push(color);
        index
    }

    /// Clear all bodies
    pub fn clear(&mut self) {
        self.positions.clear();
        self.rotations.clear();
        self.linear_velocities.clear();
        self.angular_velocities.clear();
        self.masses.clear();
        self.shape_types.clear();
        self.radii.clear();
        self.colors.clear();
    }

    /// Get cube indices
    pub fn cube_indices(&self) -> Vec<usize> {
        self.shape_types.iter().enumerate()
            .filter(|(_, &t)| t == 0)
            .map(|(i, _)| i)
            .collect()
    }

    /// Get sphere indices
    pub fn sphere_indices(&self) -> Vec<usize> {
        self.shape_types.iter().enumerate()
            .filter(|(_, &t)| t == 1)
            .map(|(i, _)| i)
            .collect()
    }
}
