# Sample Rendering

This guide covers the basics of simulating physics and rendering frames.

## Creating a Simulator

After building a scene, create a simulator:

```python
import physobx

scene = physobx.Scene()
# ... add objects to scene ...

sim = physobx.Simulator(scene, width=1920, height=1080)
```

### Resolution Options

| Resolution | Width | Height | Use Case |
|------------|-------|--------|----------|
| 720p | 1280 | 720 | Quick previews |
| 1080p | 1920 | 1080 | Standard video |
| 1440p | 2560 | 1440 | High quality |
| 4K | 3840 | 2160 | Ultra HD |

```python
# 1080p (default)
sim = physobx.Simulator(scene, width=1920, height=1080)

# 4K
sim = physobx.Simulator(scene, width=3840, height=2160)

# Custom aspect ratio (9:16 portrait for Instagram)
sim = physobx.Simulator(scene, width=2160, height=3840)
```

## Camera Setup

Position the camera to view your scene:

```python
sim.set_camera(eye, target)
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `eye` | [x, y, z] | Camera position |
| `target` | [x, y, z] | Point the camera looks at |

**Example:**

```python
# Camera at (30, 20, 30) looking at (0, 5, 0)
sim.set_camera([30, 20, 30], [0, 5, 0])
```

### Camera Tips

- **Higher Y** = looking down at the scene
- **Further from origin** = wider view
- **Target Y** = vertical center of view

```python
# Close-up, low angle
sim.set_camera([15, 5, 15], [0, 3, 0])

# Wide shot, bird's eye
sim.set_camera([80, 60, 80], [0, 10, 0])

# Side view
sim.set_camera([50, 20, 0], [0, 10, 0])
```

## Physics Simulation

### Step the Simulation

```python
sim.step(dt)
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `dt` | float | Time step in seconds |

**Standard usage (60 fps):**

```python
for frame in range(600):  # 10 seconds
    sim.step(1.0 / 60.0)
```

### Get Simulation Time

```python
current_time = sim.time()
print(f"Simulation time: {current_time:.2f}s")
```

### Get Body Data

```python
# Get all positions as NumPy array (N, 3)
positions = sim.get_positions()

# Get all rotations as quaternions (N, 4)
rotations = sim.get_rotations()

# Get shape types (0=cube, 1=sphere)
types = sim.get_shape_types()
```

## Rendering Frames

### Save as PNG

```python
sim.save_png("frame_0001.png")
```

### Get as NumPy Array

```python
# Returns (height, width, 4) RGBA array
frame = sim.render_frame()
```

### Get Dimensions

```python
width, height = sim.dimensions()
```

## Complete Rendering Loop

```python
import physobx
import os

# Setup
scene = physobx.Scene()
scene.add_ground(0.0, 50.0)
scene.add_cube_grid([0, 10, 0], 1.1, [5, 5, 5], 0.5, 1.0)
scene.add_sphere_with_velocity([20, 8, 0], [-40, 0, 0], 2.0, 50.0)

sim = physobx.Simulator(scene, width=1920, height=1080)
sim.set_camera([25, 15, 25], [0, 5, 0])

# Create output directory
output_dir = "./render/my_scene"
os.makedirs(output_dir, exist_ok=True)

# Render loop
fps = 60
duration = 5.0  # seconds
total_frames = int(fps * duration)

for frame in range(total_frames):
    # Step physics
    sim.step(1.0 / fps)

    # Save frame
    filename = f"{output_dir}/frame_{frame:04d}.png"
    sim.save_png(filename)

    # Progress
    if frame % 60 == 0:
        print(f"Frame {frame}/{total_frames}")

print("Done!")
```

## Converting to Video

Use ffmpeg to convert frames to MP4:

```bash
ffmpeg -framerate 60 -i frame_%04d.png -c:v libx264 -pix_fmt yuv420p -crf 18 video.mp4
```

### Python Helper

```python
import subprocess

def create_video(frame_dir, output_path, fps=60):
    cmd = [
        "ffmpeg", "-y",
        "-framerate", str(fps),
        "-i", f"{frame_dir}/frame_%04d.png",
        "-c:v", "libx264",
        "-pix_fmt", "yuv420p",
        "-crf", "18",
        output_path
    ]
    subprocess.run(cmd, capture_output=True)
    print(f"Video saved: {output_path}")

# Usage
create_video("./render/my_scene", "./render/my_scene/video.mp4")
```

## Quality Settings

### ffmpeg CRF Values

| CRF | Quality | File Size |
|-----|---------|-----------|
| 15 | Very high | Large |
| 18 | High (recommended) | Medium |
| 23 | Medium | Smaller |
| 28 | Low | Smallest |

### High Quality 4K Export

```bash
ffmpeg -framerate 60 -i frame_%04d.png \
  -c:v libx264 -preset slow -crf 15 \
  -pix_fmt yuv420p video_4k.mp4
```

## Example: Full Script

```python
"""Simple tower destruction render."""
import physobx
import os
import subprocess

def main():
    # Create scene
    scene = physobx.Scene()
    scene.add_ground(0.0, 50.0)

    # Tower
    for y in range(15):
        for x in range(4):
            for z in range(4):
                t = y / 15
                color = [0.8, 0.4 + t * 0.4, 0.2]
                scene.add_cube_colored(
                    [x * 1.1 - 1.5, y * 1.1 + 0.55, z * 1.1 - 1.5],
                    0.5, 1.0, color
                )

    # Wrecking ball
    scene.add_sphere_with_velocity_colored(
        [25, 8, 0], [-45, 0, 0], 2.5, 80, [0.3, 0.3, 0.9]
    )

    cubes, spheres = scene.shape_counts()
    print(f"Scene: {cubes} cubes, {spheres} balls")

    # Setup renderer
    output_dir = "./render/tower_demo"
    os.makedirs(output_dir, exist_ok=True)

    sim = physobx.Simulator(scene, width=1920, height=1080)
    sim.set_camera([20, 12, 20], [0, 6, 0])

    # Render
    fps, duration = 60, 6.0
    total_frames = int(fps * duration)

    print(f"Rendering {total_frames} frames...")
    for frame in range(total_frames):
        sim.step(1.0 / fps)
        sim.save_png(f"{output_dir}/frame_{frame:04d}.png")
        if frame % 60 == 0:
            print(f"  Frame {frame}/{total_frames}")

    # Create video
    print("Creating video...")
    subprocess.run([
        "ffmpeg", "-y", "-framerate", "60",
        "-i", f"{output_dir}/frame_%04d.png",
        "-c:v", "libx264", "-pix_fmt", "yuv420p", "-crf", "18",
        f"{output_dir}/video.mp4"
    ], capture_output=True)

    print(f"Done! Video: {output_dir}/video.mp4")

if __name__ == "__main__":
    main()
```

## Next Steps

- [Complex Rendering](complex-rendering.md) - Large-scale destruction scenes
- [Instagram Rendering](instagram-rendering.md) - 4K portrait videos
