//! Camera for 3D rendering

use bytemuck::{Pod, Zeroable};
use nalgebra::{Matrix4, Point3, Vector3};

/// Camera uniform data for GPU
#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct CameraUniform {
    pub view_proj: [[f32; 4]; 4],
    pub view: [[f32; 4]; 4],
    pub proj: [[f32; 4]; 4],
    pub eye_position: [f32; 4],
}

/// 3D camera with perspective projection
#[derive(Debug, Clone)]
pub struct Camera {
    /// Camera position
    pub eye: Point3<f32>,
    /// Look-at target
    pub target: Point3<f32>,
    /// Up vector
    pub up: Vector3<f32>,
    /// Field of view in radians
    pub fov_y: f32,
    /// Aspect ratio (width / height)
    pub aspect: f32,
    /// Near clipping plane
    pub near: f32,
    /// Far clipping plane
    pub far: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            eye: Point3::new(0.0, 20.0, 50.0),
            target: Point3::new(0.0, 5.0, 0.0),
            up: Vector3::new(0.0, 1.0, 0.0),
            fov_y: std::f32::consts::FRAC_PI_4, // 45 degrees
            aspect: 16.0 / 9.0,
            near: 0.1,
            far: 1000.0,
        }
    }
}

impl Camera {
    /// Create a new camera
    pub fn new(eye: [f32; 3], target: [f32; 3], aspect: f32) -> Self {
        Self {
            eye: Point3::from(eye),
            target: Point3::from(target),
            aspect,
            ..Default::default()
        }
    }

    /// Set aspect ratio from width and height
    pub fn set_aspect(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }

    /// Get view matrix
    pub fn view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(&self.eye, &self.target, &self.up)
    }

    /// Get projection matrix
    pub fn projection_matrix(&self) -> Matrix4<f32> {
        Matrix4::new_perspective(self.aspect, self.fov_y, self.near, self.far)
    }

    /// Get combined view-projection matrix
    pub fn view_projection_matrix(&self) -> Matrix4<f32> {
        self.projection_matrix() * self.view_matrix()
    }

    /// Get camera uniform for GPU
    pub fn uniform(&self) -> CameraUniform {
        let view = self.view_matrix();
        let proj = self.projection_matrix();
        let view_proj = proj * view;

        CameraUniform {
            view_proj: matrix_to_array(view_proj),
            view: matrix_to_array(view),
            proj: matrix_to_array(proj),
            eye_position: [self.eye.x, self.eye.y, self.eye.z, 1.0],
        }
    }
}

/// Convert nalgebra Matrix4 to array for GPU
fn matrix_to_array(m: Matrix4<f32>) -> [[f32; 4]; 4] {
    [
        [m[(0, 0)], m[(1, 0)], m[(2, 0)], m[(3, 0)]],
        [m[(0, 1)], m[(1, 1)], m[(2, 1)], m[(3, 1)]],
        [m[(0, 2)], m[(1, 2)], m[(2, 2)], m[(3, 2)]],
        [m[(0, 3)], m[(1, 3)], m[(2, 3)], m[(3, 3)]],
    ]
}
