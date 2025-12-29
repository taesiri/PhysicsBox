# Complex Rendering

This guide covers building large-scale destruction scenes with thousands of objects.

## Performance Considerations

### Object Limits

| Object Count | Physics FPS | Render FPS (1080p) |
|--------------|-------------|-------------------|
| 1,000 | ~400 | ~200 |
| 3,000 | ~200 | ~120 |
| 5,000 | ~150 | ~100 |
| 7,000 | ~100 | ~70 |
| 10,000 | ~80 | ~50 |

### Tips for Large Scenes

1. **Use `--release` builds** - Always compile with optimizations
2. **Batch object creation** - Build scenes before simulation
3. **Adjust ground size** - Match ground to scene extent
4. **Camera distance** - Further cameras render faster (less overdraw)

## Building Complex Structures

### Towers

```python
def build_tower(scene, base_x, base_z, width, height, color_func):
    """Build a tower at the given position."""
    cube_count = 0
    for y in range(height):
        # Optional taper
        layer_width = max(2, width - y // 10)
        offset = (width - layer_width) / 2

        for x in range(layer_width):
            for z in range(layer_width):
                color = color_func(y, height)
                scene.add_cube_colored(
                    [base_x + (x + offset) * 1.05,
                     y * 1.05 + 0.525,
                     base_z + (z + offset) * 1.05],
                    0.5, 1.0, color
                )
                cube_count += 1
    return cube_count

# Usage
def rainbow_color(y, height):
    import math
    hue = y / height
    return [
        0.5 + 0.5 * math.sin(hue * 2 * math.pi),
        0.5 + 0.5 * math.sin(hue * 2 * math.pi + 2.09),
        0.5 + 0.5 * math.sin(hue * 2 * math.pi + 4.19),
    ]

count = build_tower(scene, 0, 0, 6, 40, rainbow_color)
```

### Walls

```python
def build_wall(scene, start_x, end_x, z, height, thickness=1):
    """Build a wall along the X axis."""
    cube_count = 0
    length = int((end_x - start_x) / 1.05)

    for y in range(height):
        for x in range(length):
            for t in range(thickness):
                # Brick pattern color
                if (x + y) % 2 == 0:
                    color = [0.7, 0.3, 0.2]  # Dark brick
                else:
                    color = [0.8, 0.4, 0.3]  # Light brick

                scene.add_cube_colored(
                    [start_x + x * 1.05,
                     y * 1.05 + 0.525,
                     z + t * 1.05],
                    0.5, 1.0, color
                )
                cube_count += 1
    return cube_count
```

### Pyramids

```python
def build_pyramid(scene, center_x, center_z, base_size):
    """Build a pyramid structure."""
    cube_count = 0

    for y in range(base_size):
        layer_size = base_size - y
        offset = y / 2

        for x in range(layer_size):
            for z in range(layer_size):
                # Sandy gradient
                t = y / base_size
                color = [0.85 - t * 0.2, 0.65 - t * 0.15, 0.25 + t * 0.1]

                scene.add_cube_colored(
                    [center_x + (x + offset) * 1.05 - base_size * 0.525,
                     y * 1.05 + 0.525,
                     center_z + (z + offset) * 1.05 - base_size * 0.525],
                    0.5, 1.2, color
                )
                cube_count += 1
    return cube_count

# 25-layer pyramid = 5525 cubes
count = build_pyramid(scene, 0, 0, 25)
```

### Circular Structures (Stadiums, Coliseums)

```python
import math

def build_circular_wall(scene, radius, height, sections=32):
    """Build a circular wall."""
    cube_count = 0

    for section in range(sections):
        angle = section * 2 * math.pi / sections

        x = math.cos(angle) * radius
        z = math.sin(angle) * radius

        # Color varies by section
        hue = section / sections
        color = [
            0.5 + 0.4 * math.sin(hue * 2 * math.pi),
            0.5 + 0.4 * math.sin(hue * 2 * math.pi + 2.09),
            0.5 + 0.4 * math.sin(hue * 2 * math.pi + 4.19),
        ]

        for y in range(height):
            scene.add_cube_colored(
                [x, y * 1.05 + 0.525, z],
                0.5, 0.8, color
            )
            cube_count += 1

    return cube_count
```

## Destruction Patterns

### Ring Attack

Wrecking balls from all directions:

```python
import math

def add_ring_attack(scene, num_balls, distance, height, speed):
    """Add wrecking balls attacking from a ring."""
    for i in range(num_balls):
        angle = i * 2 * math.pi / num_balls

        x = math.cos(angle) * distance
        z = math.sin(angle) * distance

        # Velocity toward center
        vel_x = -math.cos(angle) * speed
        vel_z = -math.sin(angle) * speed

        color = [0.8, 0.3 + i * 0.02, 0.2]

        scene.add_sphere_with_velocity_colored(
            [x, height, z],
            [vel_x, 0, vel_z],
            2.0, 40, color
        )

# 20 balls from 50 units away, at height 15, speed 45
add_ring_attack(scene, 20, 50, 15, 45)
```

### Aerial Bombardment

Balls falling from above:

```python
import random

def add_bombardment(scene, num_balls, spread, height, down_speed):
    """Add balls falling from above."""
    for i in range(num_balls):
        x = (i % int(num_balls**0.5) - num_balls**0.5/2) * spread
        z = (i // int(num_balls**0.5) - num_balls**0.5/2) * spread

        # Add randomness
        x += random.uniform(-spread/2, spread/2)
        z += random.uniform(-spread/2, spread/2)
        h = height + random.uniform(0, 20)

        color = [0.9, 0.4, 0.2]

        scene.add_sphere_with_velocity_colored(
            [x, h, z],
            [random.uniform(-5, 5), -down_speed, random.uniform(-5, 5)],
            2.5, 50, color
        )

# 25 balls spread 10 units apart, falling from 80 units
add_bombardment(scene, 25, 10, 80, 40)
```

### Internal Explosion

Balls bursting outward from center:

```python
import math
import random

def add_internal_explosion(scene, num_balls, center, speed_range):
    """Add balls exploding outward from a point."""
    for i in range(num_balls):
        # Random direction (spherical)
        angle_h = random.uniform(0, 2 * math.pi)
        angle_v = random.uniform(-0.3, 0.8)  # Mostly upward

        speed = random.uniform(*speed_range)
        vx = math.cos(angle_h) * math.cos(angle_v) * speed
        vy = math.sin(angle_v) * speed + 15  # Upward bias
        vz = math.sin(angle_h) * math.cos(angle_v) * speed

        color = [0.9, 0.3 + random.uniform(0, 0.3), 0.1]

        scene.add_sphere_with_velocity_colored(
            center, [vx, vy, vz],
            2.0 + random.uniform(0, 1), 45, color
        )

# 30 balls exploding from center with speed 30-50
add_internal_explosion(scene, 30, [0, 5, 0], (30, 50))
```

## Complete Example: City Destruction

```python
"""City destruction scene with multiple buildings."""
import physobx
import os
import subprocess
import math
import random

def main():
    random.seed(42)

    scene = physobx.Scene()
    scene.add_ground(0.0, 200.0)

    cube_count = 0

    # Build multiple towers in a grid
    tower_positions = [
        (0, 0, 35),
        (-15, 0, 30),
        (15, 0, 30),
        (-10, -15, 25),
        (10, -15, 25),
        (0, 15, 28),
        (-20, 10, 22),
        (20, 10, 22),
    ]

    for tx, tz, height in tower_positions:
        width = 4 + random.randint(0, 2)
        for y in range(height):
            layer_width = max(2, width - y // 12)
            offset = (width - layer_width) / 2

            for x in range(layer_width):
                for z in range(layer_width):
                    # Building colors
                    t = y / height
                    r = random.uniform(-0.05, 0.05)
                    color = [0.6 + r, 0.65 + r + t * 0.1, 0.7 + r + t * 0.15]

                    scene.add_cube_colored(
                        [tx + (x + offset) * 1.05 - width * 0.525,
                         y * 1.05 + 0.525,
                         tz + (z + offset) * 1.05 - width * 0.525],
                        0.5, 0.9, color
                    )
                    cube_count += 1

    print(f"Built city with {cube_count} cubes")

    # Add 30 wrecking balls
    for i in range(30):
        angle = i * 2 * math.pi / 30 + random.uniform(-0.1, 0.1)
        distance = 60 + random.uniform(0, 10)
        height = 8 + (i % 6) * 5

        x = math.cos(angle) * distance
        z = math.sin(angle) * distance
        speed = 45 + random.uniform(0, 10)

        hue = i / 30
        color = [
            0.6 + 0.4 * math.sin(hue * 4 * math.pi),
            0.6 + 0.4 * math.sin(hue * 4 * math.pi + 2.09),
            0.6 + 0.4 * math.sin(hue * 4 * math.pi + 4.19),
        ]

        scene.add_sphere_with_velocity_colored(
            [x, height, z],
            [-math.cos(angle) * speed, 0, -math.sin(angle) * speed],
            2.0 + random.uniform(0, 0.5), 40, color
        )

    cubes, spheres = scene.shape_counts()
    print(f"Scene: {cubes} cubes, {spheres} balls")

    # Render
    output_dir = "./render/city_destruction"
    os.makedirs(output_dir, exist_ok=True)

    sim = physobx.Simulator(scene, width=1920, height=1080)
    sim.set_camera([70, 45, 70], [0, 12, 0])

    fps, duration = 60, 10.0
    total_frames = int(fps * duration)

    print(f"Rendering {total_frames} frames...")
    for frame in range(total_frames):
        sim.step(1.0 / fps)
        sim.save_png(f"{output_dir}/frame_{frame:04d}.png")
        if frame % 60 == 0:
            print(f"  Frame {frame}/{total_frames}")

    # Video
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

## Memory Management

For very large scenes (10,000+ objects):

1. **Increase ground size** to prevent objects falling off
2. **Use appropriate camera distance** for the scene scale
3. **Consider render resolution** - 4K uses 4x memory of 1080p

## Next Steps

- [Instagram Rendering](instagram-rendering.md) - 4K portrait videos for social media
