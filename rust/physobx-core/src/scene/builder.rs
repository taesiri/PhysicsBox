//! Scene builder for constructing physics scenes

/// Configuration for a rigid body
#[derive(Debug, Clone)]
pub struct RigidBodyConfig {
    pub position: [f32; 3],
    pub rotation: [f32; 4],
    pub half_extents: [f32; 3],
    pub mass: f32,
    pub restitution: f32,
    pub friction: f32,
}

impl Default for RigidBodyConfig {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0, 1.0], // Identity quaternion
            half_extents: [0.5, 0.5, 0.5],
            mass: 1.0,
            restitution: 0.3,
            friction: 0.5,
        }
    }
}

/// Builder for constructing scenes
#[derive(Debug, Default)]
pub struct SceneBuilder {
    pub bodies: Vec<RigidBodyConfig>,
    pub ground_y: Option<f32>,
    pub ground_size: f32,
}

impl SceneBuilder {
    /// Create a new scene builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a ground plane
    pub fn add_ground(&mut self, y: f32, size: f32) -> &mut Self {
        self.ground_y = Some(y);
        self.ground_size = size;
        self
    }

    /// Add a single cube
    pub fn add_cube(
        &mut self,
        position: [f32; 3],
        half_extent: f32,
        mass: f32,
    ) -> &mut Self {
        self.bodies.push(RigidBodyConfig {
            position,
            half_extents: [half_extent, half_extent, half_extent],
            mass,
            ..Default::default()
        });
        self
    }

    /// Add a grid of cubes
    pub fn add_cube_grid(
        &mut self,
        center: [f32; 3],
        spacing: f32,
        count: [u32; 3],
        half_extent: f32,
        mass: f32,
    ) -> &mut Self {
        let [cx, cy, cz] = count;
        let offset_x = (cx as f32 - 1.0) * spacing / 2.0;
        let offset_y = (cy as f32 - 1.0) * spacing / 2.0;
        let offset_z = (cz as f32 - 1.0) * spacing / 2.0;

        for ix in 0..cx {
            for iy in 0..cy {
                for iz in 0..cz {
                    let x = center[0] + ix as f32 * spacing - offset_x;
                    let y = center[1] + iy as f32 * spacing - offset_y;
                    let z = center[2] + iz as f32 * spacing - offset_z;
                    self.add_cube([x, y, z], half_extent, mass);
                }
            }
        }
        self
    }
}
