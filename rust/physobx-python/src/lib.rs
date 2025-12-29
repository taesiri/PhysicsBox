//! Python bindings for Physobx physics sandbox

use pyo3::prelude::*;
use pyo3::exceptions::PyRuntimeError;
use numpy::{PyArray1, PyArray2, PyArray3, PyArrayMethods, ToPyArray};
use physobx_core::{SceneBuilder, Simulator as CoreSimulator};
use physobx_core::gpu::Renderer;

/// Get the library version
#[pyfunction]
fn version() -> &'static str {
    physobx_core::version()
}

/// Python wrapper for SceneBuilder
#[pyclass(name = "Scene")]
pub struct PyScene {
    inner: SceneBuilder,
}

#[pymethods]
impl PyScene {
    #[new]
    fn new() -> Self {
        Self {
            inner: SceneBuilder::new(),
        }
    }

    /// Add a ground plane at the given Y position
    fn add_ground(&mut self, y: f32, size: f32) {
        self.inner.add_ground(y, size);
    }

    /// Add a single cube
    fn add_cube(&mut self, position: [f32; 3], half_extent: f32, mass: f32) {
        self.inner.add_cube(position, half_extent, mass);
    }

    /// Add a single cube with custom color
    #[pyo3(signature = (position, half_extent, mass, color))]
    fn add_cube_colored(&mut self, position: [f32; 3], half_extent: f32, mass: f32, color: [f32; 3]) {
        self.inner.add_cube_colored(position, half_extent, mass, color);
    }

    /// Add a grid of cubes
    #[pyo3(signature = (center, spacing, count, half_extent, mass))]
    fn add_cube_grid(
        &mut self,
        center: [f32; 3],
        spacing: f32,
        count: [u32; 3],
        half_extent: f32,
        mass: f32,
    ) {
        self.inner.add_cube_grid(center, spacing, count, half_extent, mass);
    }

    /// Add a single sphere
    fn add_sphere(&mut self, position: [f32; 3], radius: f32, mass: f32) {
        self.inner.add_sphere(position, radius, mass);
    }

    /// Add a single sphere with custom color
    #[pyo3(signature = (position, radius, mass, color))]
    fn add_sphere_colored(&mut self, position: [f32; 3], radius: f32, mass: f32, color: [f32; 3]) {
        self.inner.add_sphere_colored(position, radius, mass, color);
    }

    /// Add a sphere with initial velocity
    #[pyo3(signature = (position, velocity, radius, mass))]
    fn add_sphere_with_velocity(
        &mut self,
        position: [f32; 3],
        velocity: [f32; 3],
        radius: f32,
        mass: f32,
    ) {
        self.inner.add_sphere_with_velocity(position, velocity, radius, mass);
    }

    /// Add a sphere with initial velocity and custom color
    #[pyo3(signature = (position, velocity, radius, mass, color))]
    fn add_sphere_with_velocity_colored(
        &mut self,
        position: [f32; 3],
        velocity: [f32; 3],
        radius: f32,
        mass: f32,
        color: [f32; 3],
    ) {
        self.inner.add_sphere_with_velocity_colored(position, velocity, radius, mass, color);
    }

    /// Get the number of bodies in the scene
    fn body_count(&self) -> usize {
        self.inner.bodies.len()
    }

    /// Get counts of (cubes, spheres)
    fn shape_counts(&self) -> (usize, usize) {
        self.inner.shape_counts()
    }
}

/// Python wrapper for Simulator with optional rendering
#[pyclass(name = "Simulator")]
pub struct PySimulator {
    inner: CoreSimulator,
    renderer: Option<Renderer>,
    half_extent: f32,
}

#[pymethods]
impl PySimulator {
    /// Create a new simulator
    ///
    /// Args:
    ///     scene: The scene to simulate
    ///     width: Render width (default 1920)
    ///     height: Render height (default 1080)
    #[new]
    #[pyo3(signature = (scene, width=1920, height=1080))]
    fn new(scene: &PyScene, width: u32, height: u32) -> PyResult<Self> {
        // Get half_extent from first body or default
        let half_extent = scene.inner.bodies.first()
            .map(|b| b.half_extents[0])
            .unwrap_or(0.5);

        let max_instances = scene.inner.bodies.len().max(1000) as u32;

        // Get ground parameters from scene
        let ground_y = scene.inner.ground_y.unwrap_or(0.0);
        let ground_size = scene.inner.ground_size.max(50.0);

        // Create renderer with ground parameters
        let renderer = Renderer::new(width, height, max_instances, half_extent, ground_y, ground_size)
            .map_err(|e| PyRuntimeError::new_err(format!("GPU initialization failed: {}", e)))?;

        Ok(Self {
            inner: CoreSimulator::new(&scene.inner),
            renderer: Some(renderer),
            half_extent,
        })
    }

    /// Step the physics simulation
    ///
    /// Args:
    ///     dt: Time step in seconds
    ///     substeps: Number of substeps (default 1). Higher values improve
    ///               collision accuracy for fast-moving objects.
    #[pyo3(signature = (dt, substeps=1))]
    fn step(&mut self, dt: f32, substeps: u32) {
        let sub_dt = dt / substeps as f32;
        for _ in 0..substeps {
            self.inner.step(sub_dt);
        }
    }

    /// Get the current simulation time
    fn time(&self) -> f32 {
        self.inner.time
    }

    /// Get the number of bodies
    fn body_count(&self) -> usize {
        self.inner.body_count()
    }

    /// Get positions as a NumPy array (N, 3)
    fn get_positions<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray2<f32>> {
        let positions = self.inner.positions();
        let n = positions.len();
        let flat: Vec<f32> = positions.iter()
            .flat_map(|p| p.iter().copied())
            .collect();
        flat.to_pyarray(py).reshape([n, 3]).unwrap()
    }

    /// Get rotations as a NumPy array (N, 4)
    fn get_rotations<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray2<f32>> {
        let rotations = self.inner.rotations();
        let n = rotations.len();
        let flat: Vec<f32> = rotations.iter()
            .flat_map(|r| r.iter().copied())
            .collect();
        flat.to_pyarray(py).reshape([n, 4]).unwrap()
    }

    /// Set camera position and target
    #[pyo3(signature = (eye, target))]
    fn set_camera(&mut self, eye: [f32; 3], target: [f32; 3]) -> PyResult<()> {
        if let Some(ref mut renderer) = self.renderer {
            renderer.set_camera(eye, target);
            Ok(())
        } else {
            Err(PyRuntimeError::new_err("Renderer not available"))
        }
    }

    /// Render a frame and return as NumPy array (H, W, 4)
    fn render_frame<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyArray3<u8>>> {
        let renderer = self.renderer.as_ref()
            .ok_or_else(|| PyRuntimeError::new_err("Renderer not available"))?;

        // Get separated cube and sphere data (with colors)
        let (cube_positions, cube_rotations, cube_colors) = self.inner.cube_data();
        let (sphere_positions, sphere_radii, sphere_colors) = self.inner.sphere_data();

        let pixels = renderer.render_frame_with_shapes(
            &cube_positions,
            &cube_rotations,
            &cube_colors,
            &sphere_positions,
            &sphere_radii,
            &sphere_colors,
        );
        let (width, height) = renderer.dimensions();

        Ok(pixels.to_pyarray(py).reshape([height as usize, width as usize, 4]).unwrap())
    }

    /// Save current frame as PNG
    fn save_png(&self, path: &str) -> PyResult<()> {
        let renderer = self.renderer.as_ref()
            .ok_or_else(|| PyRuntimeError::new_err("Renderer not available"))?;

        // Get separated cube and sphere data (with colors)
        let (cube_positions, cube_rotations, cube_colors) = self.inner.cube_data();
        let (sphere_positions, sphere_radii, sphere_colors) = self.inner.sphere_data();

        renderer.save_png_with_shapes(
            &cube_positions,
            &cube_rotations,
            &cube_colors,
            &sphere_positions,
            &sphere_radii,
            &sphere_colors,
            path,
        ).map_err(|e| PyRuntimeError::new_err(format!("Failed to save PNG: {}", e)))
    }

    /// Get shape types as NumPy array (0=cube, 1=sphere)
    fn get_shape_types<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<u8>> {
        self.inner.shape_types().to_pyarray(py)
    }

    /// Get render dimensions
    fn dimensions(&self) -> PyResult<(u32, u32)> {
        let renderer = self.renderer.as_ref()
            .ok_or_else(|| PyRuntimeError::new_err("Renderer not available"))?;
        Ok(renderer.dimensions())
    }
}

/// Physobx Python module
#[pymodule]
fn physobx(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Initialize logging
    let _ = env_logger::try_init();

    m.add_function(wrap_pyfunction!(version, m)?)?;
    m.add_class::<PyScene>()?;
    m.add_class::<PySimulator>()?;
    Ok(())
}
