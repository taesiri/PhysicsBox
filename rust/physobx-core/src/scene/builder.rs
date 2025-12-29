//! Scene builder for constructing physics scenes

/// Shape type for rigid bodies
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShapeType {
    Cube,
    Sphere,
}

/// Configuration for a rigid body
#[derive(Debug, Clone)]
pub struct RigidBodyConfig {
    pub position: [f32; 3],
    pub rotation: [f32; 4],
    pub velocity: [f32; 3],
    pub half_extents: [f32; 3],
    pub radius: f32,
    pub shape: ShapeType,
    pub mass: f32,
    pub restitution: f32,
    pub friction: f32,
    pub color: [f32; 3],  // RGB color
}

impl Default for RigidBodyConfig {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0, 1.0], // Identity quaternion
            velocity: [0.0, 0.0, 0.0],
            half_extents: [0.5, 0.5, 0.5],
            radius: 0.5,
            shape: ShapeType::Cube,
            mass: 1.0,
            restitution: 0.3,
            friction: 0.5,
            color: [0.82, 0.32, 0.12],  // Default terracotta
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

    /// Add a single cube with custom color
    pub fn add_cube_colored(
        &mut self,
        position: [f32; 3],
        half_extent: f32,
        mass: f32,
        color: [f32; 3],
    ) -> &mut Self {
        self.bodies.push(RigidBodyConfig {
            position,
            half_extents: [half_extent, half_extent, half_extent],
            mass,
            color,
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

    /// Add a single sphere
    pub fn add_sphere(
        &mut self,
        position: [f32; 3],
        radius: f32,
        mass: f32,
    ) -> &mut Self {
        self.bodies.push(RigidBodyConfig {
            position,
            radius,
            shape: ShapeType::Sphere,
            mass,
            restitution: 0.6,  // Spheres bounce more
            color: [0.35, 0.5, 0.75],  // Default blue for spheres
            ..Default::default()
        });
        self
    }

    /// Add a single sphere with custom color
    pub fn add_sphere_colored(
        &mut self,
        position: [f32; 3],
        radius: f32,
        mass: f32,
        color: [f32; 3],
    ) -> &mut Self {
        self.bodies.push(RigidBodyConfig {
            position,
            radius,
            shape: ShapeType::Sphere,
            mass,
            restitution: 0.6,
            color,
            ..Default::default()
        });
        self
    }

    /// Add a sphere with initial velocity
    pub fn add_sphere_with_velocity(
        &mut self,
        position: [f32; 3],
        velocity: [f32; 3],
        radius: f32,
        mass: f32,
    ) -> &mut Self {
        self.bodies.push(RigidBodyConfig {
            position,
            velocity,
            radius,
            shape: ShapeType::Sphere,
            mass,
            restitution: 0.6,
            color: [0.35, 0.5, 0.75],  // Default blue
            ..Default::default()
        });
        self
    }

    /// Add a sphere with initial velocity and custom color
    pub fn add_sphere_with_velocity_colored(
        &mut self,
        position: [f32; 3],
        velocity: [f32; 3],
        radius: f32,
        mass: f32,
        color: [f32; 3],
    ) -> &mut Self {
        self.bodies.push(RigidBodyConfig {
            position,
            velocity,
            radius,
            shape: ShapeType::Sphere,
            mass,
            restitution: 0.6,
            color,
            ..Default::default()
        });
        self
    }

    /// Get counts of each shape type
    pub fn shape_counts(&self) -> (usize, usize) {
        let cubes = self.bodies.iter().filter(|b| b.shape == ShapeType::Cube).count();
        let spheres = self.bodies.iter().filter(|b| b.shape == ShapeType::Sphere).count();
        (cubes, spheres)
    }
}
