//! Python bindings for Physobx physics sandbox

use pyo3::prelude::*;
use pyo3::exceptions::PyRuntimeError;
use numpy::{PyArray2, PyArray3, PyArrayMethods, ToPyArray};
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

    /// Get the number of bodies in the scene
    fn body_count(&self) -> usize {
        self.inner.bodies.len()
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

        // Create renderer
        let renderer = Renderer::new(width, height, max_instances, half_extent)
            .map_err(|e| PyRuntimeError::new_err(format!("GPU initialization failed: {}", e)))?;

        Ok(Self {
            inner: CoreSimulator::new(&scene.inner),
            renderer: Some(renderer),
            half_extent,
        })
    }

    /// Step the physics simulation
    fn step(&mut self, dt: f32) {
        self.inner.step(dt);
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

        let positions = self.inner.positions();
        let rotations = self.inner.rotations();

        let pixels = renderer.render_frame(positions, rotations);
        let (width, height) = renderer.dimensions();

        Ok(pixels.to_pyarray(py).reshape([height as usize, width as usize, 4]).unwrap())
    }

    /// Save current frame as PNG
    fn save_png(&self, path: &str) -> PyResult<()> {
        let renderer = self.renderer.as_ref()
            .ok_or_else(|| PyRuntimeError::new_err("Renderer not available"))?;

        let positions = self.inner.positions();
        let rotations = self.inner.rotations();

        renderer.save_png(positions, rotations, path)
            .map_err(|e| PyRuntimeError::new_err(format!("Failed to save PNG: {}", e)))
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
