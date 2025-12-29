# Scene Setup

This guide covers creating physics scenes with cubes, spheres, and ground planes.

## Basic Concepts

A **Scene** contains:
- A ground plane (optional but recommended)
- Rigid bodies (cubes and spheres)
- Each body has position, rotation, mass, and color

## Creating a Scene

```python
import physobx

scene = physobx.Scene()
```

## Adding a Ground Plane

The ground plane provides a surface for objects to land on:

```python
scene.add_ground(y_position, size)
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `y_position` | float | Height of the ground (usually 0.0) |
| `size` | float | Half-extent of the ground plane |

**Example:**

```python
# Ground at y=0, extending 100 units in each direction
scene.add_ground(0.0, 100.0)
```

## Adding Cubes

### Single Cube

```python
scene.add_cube(position, half_extent, mass)
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `position` | [x, y, z] | Center position |
| `half_extent` | float | Half the cube's side length |
| `mass` | float | Mass in kg |

**Example:**

```python
# 1-unit cube at position (0, 5, 0) with mass 1.0
scene.add_cube([0, 5, 0], 0.5, 1.0)
```

### Cube with Color

```python
scene.add_cube_colored(position, half_extent, mass, color)
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `color` | [r, g, b] | RGB color (0.0 to 1.0 each) |

**Example:**

```python
# Red cube
scene.add_cube_colored([0, 5, 0], 0.5, 1.0, [1.0, 0.2, 0.2])

# Blue cube
scene.add_cube_colored([2, 5, 0], 0.5, 1.0, [0.2, 0.4, 0.9])
```

### Cube Grid

For creating many cubes at once:

```python
scene.add_cube_grid(center, spacing, count, half_extent, mass)
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `center` | [x, y, z] | Center of the grid |
| `spacing` | float | Distance between cube centers |
| `count` | [nx, ny, nz] | Number of cubes in each dimension |
| `half_extent` | float | Half-size of each cube |
| `mass` | float | Mass of each cube |

**Example:**

```python
# 10x10x10 grid (1000 cubes)
scene.add_cube_grid(
    center=[0, 10, 0],
    spacing=1.1,
    count=[10, 10, 10],
    half_extent=0.5,
    mass=1.0
)
```

## Adding Spheres

### Single Sphere

```python
scene.add_sphere(position, radius, mass)
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `position` | [x, y, z] | Center position |
| `radius` | float | Sphere radius |
| `mass` | float | Mass in kg |

**Example:**

```python
# Sphere at (0, 10, 0) with radius 1.0
scene.add_sphere([0, 10, 0], 1.0, 5.0)
```

### Sphere with Color

```python
scene.add_sphere_colored(position, radius, mass, color)
```

**Example:**

```python
# Green sphere
scene.add_sphere_colored([0, 10, 0], 1.0, 5.0, [0.2, 0.9, 0.3])
```

### Sphere with Velocity (Projectile)

For creating moving spheres (wrecking balls, projectiles):

```python
scene.add_sphere_with_velocity(position, velocity, radius, mass)
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `velocity` | [vx, vy, vz] | Initial velocity in m/s |

**Example:**

```python
# Wrecking ball moving toward origin
scene.add_sphere_with_velocity(
    position=[30, 10, 0],
    velocity=[-40, 0, 0],  # Moving left at 40 m/s
    radius=2.0,
    mass=50.0
)
```

### Sphere with Velocity and Color

```python
scene.add_sphere_with_velocity_colored(position, velocity, radius, mass, color)
```

**Example:**

```python
# Orange wrecking ball
scene.add_sphere_with_velocity_colored(
    [30, 10, 0],
    [-40, 0, 0],
    2.0,
    50.0,
    [0.9, 0.5, 0.1]
)
```

## Scene Information

### Get Body Count

```python
total = scene.body_count()
print(f"Total bodies: {total}")
```

### Get Shape Counts

```python
cubes, spheres = scene.shape_counts()
print(f"Cubes: {cubes}, Spheres: {spheres}")
```

## Color Tips

Colors are RGB values from 0.0 to 1.0:

| Color | RGB Value |
|-------|-----------|
| Red | `[1.0, 0.2, 0.2]` |
| Green | `[0.2, 0.9, 0.3]` |
| Blue | `[0.2, 0.4, 0.9]` |
| Yellow | `[0.9, 0.8, 0.2]` |
| Orange | `[0.9, 0.5, 0.1]` |
| Purple | `[0.7, 0.3, 0.9]` |
| White | `[0.9, 0.9, 0.9]` |
| Gray | `[0.5, 0.5, 0.5]` |

### Rainbow Gradient

```python
import math

for i in range(n):
    hue = i / n
    color = [
        0.5 + 0.5 * math.sin(hue * 2 * math.pi),
        0.5 + 0.5 * math.sin(hue * 2 * math.pi + 2.09),
        0.5 + 0.5 * math.sin(hue * 2 * math.pi + 4.19),
    ]
    scene.add_cube_colored([i, 0.5, 0], 0.5, 1.0, color)
```

## Example: Tower with Wrecking Ball

```python
import physobx

scene = physobx.Scene()
scene.add_ground(0.0, 50.0)

# Build a 5x5x20 tower
for y in range(20):
    for x in range(5):
        for z in range(5):
            # Color gradient from red (bottom) to blue (top)
            t = y / 20
            color = [1.0 - t, 0.3, t]
            scene.add_cube_colored(
                [x * 1.05 - 2, y * 1.05 + 0.525, z * 1.05 - 2],
                0.5, 1.0, color
            )

# Add wrecking ball
scene.add_sphere_with_velocity_colored(
    [30, 10, 0],   # Start position
    [-50, 0, 0],   # Velocity (toward tower)
    3.0,           # Radius
    100.0,         # Mass
    [0.8, 0.4, 0.1]  # Orange color
)

cubes, spheres = scene.shape_counts()
print(f"Scene: {cubes} cubes, {spheres} spheres")
```

## Next Steps

- [Sample Rendering](sample-rendering.md) - Render your scene to images/video
- [Complex Rendering](complex-rendering.md) - Build large-scale destruction scenes
