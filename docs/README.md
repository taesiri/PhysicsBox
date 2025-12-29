# Physobx Documentation

Physobx is a high-performance physics sandbox with GPU-accelerated rendering, designed for creating satisfying destruction simulations and physics-based videos.

## Features

- **Rapier 3D Physics** - Fast, stable rigid body simulation
- **GPU Rendering** - Metal-accelerated headless rendering via wgpu
- **Shadow Mapping** - Soft shadows with PCF filtering
- **HDR Pipeline** - ACES tonemapping for cinematic visuals
- **Python Bindings** - Easy-to-use API via PyO3
- **Multiple Resolutions** - From 1080p to 4K, any aspect ratio

## Documentation

| Guide | Description |
|-------|-------------|
| [Installation](installation.md) | Setup and dependencies |
| [Scene Setup](scene-setup.md) | Creating scenes with cubes and spheres |
| [Sample Rendering](sample-rendering.md) | Basic rendering workflow |
| [Complex Rendering](complex-rendering.md) | Large-scale destruction scenes |
| [Instagram Rendering](instagram-rendering.md) | 4K portrait videos for social media |

## Quick Start

```python
import physobx

# Create a scene
scene = physobx.Scene()
scene.add_ground(0.0, 50.0)
scene.add_cube_grid([0, 10, 0], 1.1, [5, 5, 5], 0.5, 1.0)
scene.add_sphere_with_velocity([0, 20, 10], [0, -30, -20], 2.0, 50)

# Simulate and render
sim = physobx.Simulator(scene, width=1920, height=1080)
sim.set_camera([20, 15, 20], [0, 5, 0])

for i in range(300):
    sim.step(1/60)
    sim.save_png(f"frame_{i:04d}.png")
```

## Architecture

```
physobx/
├── rust/
│   ├── physobx-core/     # Core Rust library
│   │   ├── src/
│   │   │   ├── physics/  # Rapier integration, SOA storage
│   │   │   └── gpu/      # wgpu rendering pipeline
│   │   └── shaders/      # WGSL shaders
│   └── physobx-python/   # PyO3 bindings
├── examples/             # Example scenes
└── docs/                 # Documentation
```

## Performance

Typical performance on Apple Silicon (M1/M2/M3):

| Scene Size | Physics | Render (1080p) | Render (4K) |
|------------|---------|----------------|-------------|
| 1,000 cubes | ~400 fps | ~200 fps | ~60 fps |
| 5,000 cubes | ~150 fps | ~100 fps | ~30 fps |
| 10,000 cubes | ~80 fps | ~50 fps | ~15 fps |

## License

MIT License
