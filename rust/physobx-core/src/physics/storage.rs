//! SOA (Structure of Arrays) storage for rigid body data
//!
//! This provides cache-friendly, SIMD-optimized storage for physics state.

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

    /// Add a new rigid body
    pub fn push(
        &mut self,
        position: [f32; 3],
        rotation: [f32; 4],
        mass: f32,
    ) -> usize {
        let index = self.positions.len();
        self.positions.push(position);
        self.rotations.push(rotation);
        self.linear_velocities.push([0.0, 0.0, 0.0]);
        self.angular_velocities.push([0.0, 0.0, 0.0]);
        self.masses.push(mass);
        index
    }

    /// Clear all bodies
    pub fn clear(&mut self) {
        self.positions.clear();
        self.rotations.clear();
        self.linear_velocities.clear();
        self.angular_velocities.clear();
        self.masses.clear();
    }
}
