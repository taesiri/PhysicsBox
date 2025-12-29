# Instagram Rendering

This guide covers creating 4K portrait videos optimized for Instagram Reels and Stories.

## Instagram Video Specs

| Platform | Aspect Ratio | Resolution | Max Duration |
|----------|--------------|------------|--------------|
| Reels | 9:16 | 1080x1920 or 2160x3840 | 90 seconds |
| Stories | 9:16 | 1080x1920 | 15 seconds |
| Feed | 4:5 | 1080x1350 | 60 seconds |

## 4K Portrait Setup

For highest quality Reels, use 4K portrait (2160x3840):

```python
sim = physobx.Simulator(scene, width=2160, height=3840)
```

## Camera Considerations

Portrait orientation requires different camera positioning:

### Vertical Scenes (Towers, Waterfalls)

```python
# Looking up at a tall structure
sim.set_camera([25, 30, 25], [0, 40, 0])

# Side view of tall tower
sim.set_camera([40, 35, 0], [0, 35, 0])
```

### Centered Action

```python
# Action in the middle of frame
sim.set_camera([35, 40, 35], [0, 25, 0])
```

### Low Angle (Dramatic)

```python
# Looking up for dramatic effect
sim.set_camera([30, 10, 30], [0, 30, 0])
```

## Scene Ideas for Portrait

### 1. Tower Collapse

Tall towers work perfectly for 9:16:

```python
def build_rainbow_tower(scene, height=60, width=5):
    import math

    for y in range(height):
        layer_width = max(3, width - y // 20)
        offset = (width - layer_width) / 2

        for x in range(layer_width):
            for z in range(layer_width):
                hue = y / height
                color = [
                    0.5 + 0.5 * math.sin(hue * 2 * math.pi),
                    0.5 + 0.5 * math.sin(hue * 2 * math.pi + 2.09),
                    0.5 + 0.5 * math.sin(hue * 2 * math.pi + 4.19),
                ]
                scene.add_cube_colored(
                    [(x + offset) * 1.05 - width * 0.525 + 0.525,
                     y * 1.05 + 0.525,
                     (z + offset) * 1.05 - width * 0.525 + 0.525],
                    0.5, 1.0, color
                )
```

### 2. Spiral Cascade

Spirals rising vertically fill portrait frames well:

```python
def build_spiral(scene, num_elements=300, turns=4, max_height=50):
    import math

    for i in range(num_elements):
        t = i / num_elements
        angle = t * turns * 2 * math.pi
        radius = 8 + t * 20
        height = t * max_height

        x = math.cos(angle) * radius
        z = math.sin(angle) * radius

        hue = t
        color = [
            0.4 + 0.5 * math.sin(hue * 4 * math.pi),
            0.5 + 0.4 * math.sin(hue * 4 * math.pi + 2.09),
            0.6 + 0.3 * math.sin(hue * 4 * math.pi + 4.19),
        ]

        # Stack cubes vertically at each point
        for dy in range(3):
            scene.add_cube_colored(
                [x, height + dy * 1.05 + 0.525, z],
                0.5, 0.8, color
            )
```

### 3. Waterfall Steps

Stepped platforms create vertical motion:

```python
def build_waterfall(scene, num_steps=10, cubes_per_step=100):
    import random

    for step in range(num_steps):
        platform_y = 60 - step * 5
        platform_x = -20 + step * 4

        # Platform
        for px in range(6):
            for pz in range(12):
                scene.add_cube_colored(
                    [platform_x + px * 1.05, platform_y, pz * 1.05 - 6],
                    0.5, 2.0, [0.4, 0.45, 0.5]
                )

        # Colorful cubes on top
        for row in range(2):
            for col in range(10):
                hue = (step * 10 + col) / (num_steps * 10)
                color = [
                    0.5 + 0.5 * math.sin(hue * 6 * math.pi),
                    0.5 + 0.5 * math.sin(hue * 6 * math.pi + 2.09),
                    0.5 + 0.5 * math.sin(hue * 6 * math.pi + 4.19),
                ]
                scene.add_cube_colored(
                    [platform_x + 1 + row * 1.5,
                     platform_y + 1.5 + random.uniform(0, 0.3),
                     col * 1.05 - 5],
                    0.45, 0.7, color
                )
```

### 4. Twin Towers

Two parallel towers with bridge:

```python
def build_twin_towers(scene, height=50, spacing=15):
    for tower in [-1, 1]:
        tower_x = tower * spacing / 2

        for y in range(height):
            width = max(3, 5 - y // 20)
            offset = (5 - width) / 2

            for x in range(width):
                for z in range(width):
                    t = y / height
                    if tower == -1:
                        color = [0.3 + t * 0.2, 0.5 + t * 0.3, 0.8]
                    else:
                        color = [0.8, 0.5 + t * 0.3, 0.3 + t * 0.2]

                    scene.add_cube_colored(
                        [tower_x + (x + offset) * 1.05 - 2.5,
                         y * 1.05 + 0.525,
                         (z + offset) * 1.05 - 2.5],
                        0.5, 0.9, color
                    )

    # Bridge at mid-height
    bridge_y = height // 2
    for bx in range(-6, 7):
        for bz in range(2):
            scene.add_cube_colored(
                [bx * 1.05, bridge_y * 1.05, bz * 1.05 - 0.5],
                0.5, 1.0, [0.6, 0.6, 0.65]
            )
```

## Complete Instagram Script

```python
"""Instagram 4K Portrait Video Generator"""
import physobx
import os
import subprocess
import math
import random
from datetime import datetime

def create_scene():
    """Create a visually striking vertical scene."""
    scene = physobx.Scene()
    scene.add_ground(0.0, 100.0)

    # Build tall rainbow tower
    tower_height = 70
    tower_width = 6

    for y in range(tower_height):
        width = max(3, tower_width - y // 18)
        offset = (tower_width - width) / 2

        for x in range(width):
            for z in range(width):
                hue = y / tower_height
                color = [
                    0.5 + 0.5 * math.sin(hue * 2 * math.pi),
                    0.5 + 0.5 * math.sin(hue * 2 * math.pi + 2.09),
                    0.5 + 0.5 * math.sin(hue * 2 * math.pi + 4.19),
                ]
                scene.add_cube_colored(
                    [(x + offset) * 1.05 - tower_width * 0.525 + 0.525,
                     y * 1.05 + 0.525,
                     (z + offset) * 1.05 - tower_width * 0.525 + 0.525],
                    0.5, 1.0, color
                )

    # Ring of wrecking balls
    for i in range(12):
        angle = i * 2 * math.pi / 12
        height = 15 + (i % 4) * 12
        distance = 35

        x = math.cos(angle) * distance
        z = math.sin(angle) * distance
        speed = 40

        color = [0.8, 0.3 + i * 0.04, 0.2]
        scene.add_sphere_with_velocity_colored(
            [x, height, z],
            [-math.cos(angle) * speed, 0, -math.sin(angle) * speed],
            2.0, 40, color
        )

    # Top bombers
    for i in range(6):
        x = (i % 3 - 1) * 3
        z = (i // 3 - 0.5) * 3
        scene.add_sphere_with_velocity_colored(
            [x, 90, z], [0, -45, 0], 2.5, 55, [0.9, 0.5, 0.1]
        )

    return scene

def main():
    random.seed(42)

    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    output_dir = f"./render/instagram_{timestamp}"
    os.makedirs(output_dir, exist_ok=True)

    print("Creating scene...")
    scene = create_scene()
    cubes, spheres = scene.shape_counts()
    print(f"Scene: {cubes} cubes, {spheres} balls")

    # 4K Portrait (9:16)
    print("Initializing 4K portrait renderer (2160x3840)...")
    sim = physobx.Simulator(scene, width=2160, height=3840)
    sim.set_camera([30, 40, 30], [0, 35, 0])

    # Render
    fps = 60
    duration = 10.0
    total_frames = int(fps * duration)

    print(f"Rendering {total_frames} frames...")
    for frame in range(total_frames):
        sim.step(1.0 / fps)
        sim.save_png(f"{output_dir}/frame_{frame:04d}.png")
        if frame % 60 == 0:
            print(f"  Frame {frame}/{total_frames} ({frame/total_frames*100:.0f}%)")

    # Create video
    print("Creating video...")
    video_path = f"{output_dir}/video.mp4"
    subprocess.run([
        "ffmpeg", "-y", "-framerate", "60",
        "-i", f"{output_dir}/frame_%04d.png",
        "-c:v", "libx264", "-pix_fmt", "yuv420p", "-crf", "18",
        video_path
    ], capture_output=True)

    size_mb = os.path.getsize(video_path) / (1024 * 1024)
    print(f"\nDone!")
    print(f"Video: {video_path} ({size_mb:.1f} MB)")

if __name__ == "__main__":
    main()
```

## Video Encoding Tips

### High Quality for Instagram

```bash
ffmpeg -framerate 60 -i frame_%04d.png \
  -c:v libx264 -preset slow -crf 18 \
  -pix_fmt yuv420p \
  -movflags +faststart \
  video.mp4
```

### Smaller File Size

For faster uploads:

```bash
ffmpeg -framerate 60 -i frame_%04d.png \
  -c:v libx264 -preset medium -crf 23 \
  -pix_fmt yuv420p \
  -movflags +faststart \
  video_small.mp4
```

### With Audio (Optional)

Add background music:

```bash
ffmpeg -framerate 60 -i frame_%04d.png \
  -i music.mp3 \
  -c:v libx264 -crf 18 -pix_fmt yuv420p \
  -c:a aac -b:a 192k \
  -shortest \
  video_with_audio.mp4
```

## Pre-built Instagram Scenes

Run the included Instagram test script:

```bash
python examples/instagram_4k_test.py
```

This generates 5 different 4K portrait scenes:
1. **Tower Collapse** - Rainbow gradient tower
2. **Spiral Domino** - Rising spiral cascade
3. **Cube Waterfall** - Stepped platforms
4. **Pyramid Explosion** - Giant pyramid bursting
5. **Twin Towers** - Synchronized destruction

## Performance Notes

4K portrait rendering is demanding:

| Resolution | Memory | Render Time (600 frames) |
|------------|--------|--------------------------|
| 1080x1920 | ~100 MB | ~2 minutes |
| 2160x3840 | ~400 MB | ~8 minutes |

Ensure adequate disk space (~500 MB per 10-second scene at 4K).

## Upload Checklist

Before uploading to Instagram:

- [ ] Video is 9:16 aspect ratio
- [ ] Resolution is 1080x1920 or higher
- [ ] Duration is under 90 seconds (Reels)
- [ ] File size is under 250 MB
- [ ] Frame rate is 30 or 60 fps
- [ ] No letterboxing (full bleed)
