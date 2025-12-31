# PhysicsBox

High-performance physics sandbox with Rust core, Python bindings, and Metal GPU acceleration for macOS.

## Features

- **Rapier 3D Physics**: Fast rigid body simulation with CCD (Continuous Collision Detection)
- **GPU Rendering**: wgpu-based renderer with Metal backend for headless 4K rendering
- **Shadow Mapping**: Real-time shadows with PCF soft edges
- **HDR Pipeline**: ACES tonemapping for cinematic quality
- **Python Bindings**: PyO3-based API for easy scripting
- **Instagram Ready**: 4K portrait (2160x3840) video export at 60fps

## Installation

### Prerequisites

- macOS with Metal support
- Rust toolchain
- Python 3.8+
- [uv](https://github.com/astral-sh/uv) package manager
- ffmpeg (for video encoding)

### Setup

```bash
# Clone the repository
git clone https://github.com/taesiri/PhysicsBox.git
cd PhysicsBox

# Install dependencies and build
uv sync
maturin develop --release
```

## Quick Start

```python
import physobx

# Create a scene
scene = physobx.Scene()
scene.add_ground(0.0, 50.0)

# Add some cubes
scene.add_cube([0.0, 5.0, 0.0], 0.5, 1.0)
scene.add_cube([0.0, 7.0, 0.0], 0.5, 1.0)

# Add a sphere
scene.add_sphere([2.0, 3.0, 0.0], 1.0, 2.0)

# Create simulator with 1080p resolution
sim = physobx.Simulator(scene, width=1920, height=1080)
sim.set_camera([10.0, 8.0, 10.0], [0.0, 3.0, 0.0])

# Simulate and render
for i in range(60):
    sim.step(1.0 / 60.0)

sim.save_png("output.png")
```

## Rendering Instagram Videos

### Quick Test (3 seconds)

```bash
python -c "
import physobx
import os
import subprocess

output_dir = './render/instagram_test'
os.makedirs(output_dir, exist_ok=True)

# Create scene with tower and wrecking ball
scene = physobx.Scene()
scene.add_ground(0.0, 100.0)

# Build colorful tower
for y in range(20):
    for x in range(4):
        for z in range(4):
            t = y / 20
            color = [0.9 - t * 0.4, 0.4 + t * 0.3, 0.2 + t * 0.5]
            scene.add_cube_colored(
                [x * 1.05 - 1.575, y * 1.05 + 0.525, z * 1.05 - 1.575],
                0.5, 1.0, color
            )

# Wrecking ball
scene.add_sphere_with_velocity_colored(
    [25, 10, 0], [-40, 5, 0], 3.0, 200.0, [0.3, 0.3, 0.4]
)

# 4K Portrait for Instagram
sim = physobx.Simulator(scene, width=2160, height=3840)
sim.set_camera([30, 20, 30], [0, 10, 0])

# Render frames
fps = 60
for frame in range(180):  # 3 seconds
    sim.step(1.0 / fps, 2)  # 2 substeps for accuracy
    sim.save_png(f'{output_dir}/frame_{frame:04d}.png')
    if frame % 30 == 0:
        print(f'Frame {frame}/180')

# Create video with ffmpeg
subprocess.run([
    'ffmpeg', '-y', '-framerate', '60',
    '-i', f'{output_dir}/frame_%04d.png',
    '-c:v', 'libx264', '-pix_fmt', 'yuv420p', '-crf', '18',
    f'{output_dir}/video.mp4'
])
print(f'Video saved to {output_dir}/video.mp4')
"
```

### Full Instagram Test Suite

```bash
# Run the complete physics test (3 scenes, 10 seconds each)
python examples/instagram_physics_test.py
```

This renders three scenes:
1. **Wrecking Ball vs Tower** - Massive ball destroys a 50-layer tower
2. **Ball Rain** - 50 balls raining on multi-level platforms
3. **Domino Chain** - 300-piece spiral domino with chain reaction

### Available Examples

```bash
# List all example scripts
ls examples/

# Run specific examples
python examples/falling_cubes.py      # 1000 cubes falling
python examples/cube_tower.py         # Tower collapse
python examples/multiball_chaos.py    # Multiple balls and cubes
python examples/instagram_4k_test.py  # 4K portrait scenes
```

## API Reference

### Scene

```python
scene = physobx.Scene()

# Ground plane
scene.add_ground(y_position, size)

# Cubes
scene.add_cube(position, half_extent, mass)
scene.add_cube_colored(position, half_extent, mass, [r, g, b])
scene.add_cube_grid(center, spacing, [nx, ny, nz], half_extent, mass)

# Spheres
scene.add_sphere(position, radius, mass)
scene.add_sphere_colored(position, radius, mass, [r, g, b])
scene.add_sphere_with_velocity(position, velocity, radius, mass)
scene.add_sphere_with_velocity_colored(position, velocity, radius, mass, [r, g, b])

# Get counts
cubes, spheres = scene.shape_counts()
```

### Simulator

```python
sim = physobx.Simulator(scene, width=1920, height=1080)

# Camera
sim.set_camera(eye_position, target_position)

# Physics step
sim.step(dt)                    # Single step
sim.step(dt, substeps)          # With substeps for accuracy

# Rendering
sim.save_png("frame.png")
frame = sim.render_frame()      # Returns numpy array (RGBA)

# Get physics state
positions = sim.get_positions() # numpy array (N, 3)
```

## Project Structure

```
PhysicsBox/
├── rust/
│   ├── physobx-core/          # Core Rust library
│   │   ├── src/
│   │   │   ├── physics/       # Rapier physics integration
│   │   │   └── gpu/           # wgpu rendering
│   │   └── shaders/           # WGSL shaders
│   └── physobx-python/        # PyO3 bindings
├── examples/                   # Python example scripts
├── scripts/                    # Utility scripts
└── pyproject.toml             # Python package config
```

## Performance

- **1000 cubes**: ~400+ fps physics + GPU render
- **4K rendering**: ~2-3 fps (headless Metal)
- **Shadow mapping**: 2048x2048 depth texture with PCF

## License

MIT
