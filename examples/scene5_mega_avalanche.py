"""Scene 5: Mega Avalanche - 5000+ cubes in massive structures with 50 balls."""

import physobx
import os
import math
import random
from datetime import datetime

def main():
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    output_dir = f"./render/scene5_mega_avalanche_{timestamp}"
    os.makedirs(output_dir, exist_ok=True)
    print(f"Output folder: {output_dir}")

    random.seed(123)  # Reproducible randomness

    scene = physobx.Scene()
    scene.add_ground(0.0, 300.0)

    cube_count = 0

    # Giant pyramid in center (largest structure)
    pyramid_base = 20
    print(f"Building central pyramid (base {pyramid_base})...")
    for level in range(pyramid_base):
        size = pyramid_base - level
        # Rainbow gradient by level
        hue = level / pyramid_base
        color = [
            0.5 + 0.5 * math.sin(hue * 2 * math.pi),
            0.5 + 0.5 * math.sin(hue * 2 * math.pi + 2.09),
            0.5 + 0.5 * math.sin(hue * 2 * math.pi + 4.19),
        ]

        for x in range(size):
            for z in range(size):
                scene.add_cube_colored(
                    [x * 1.05 - size * 0.525 + 0.525,
                     level * 1.05 + 0.525,
                     z * 1.05 - size * 0.525 + 0.525],
                    0.5, 0.8, color
                )
                cube_count += 1

    print(f"  Pyramid: {cube_count} cubes")

    # Four corner mega-towers
    tower_positions = [(-25, -25), (-25, 25), (25, -25), (25, 25)]
    tower_colors_base = [
        [0.9, 0.3, 0.2],   # Red
        [0.2, 0.9, 0.3],   # Green
        [0.3, 0.2, 0.9],   # Blue
        [0.9, 0.9, 0.2],   # Yellow
    ]

    for (tx, tz), base_color in zip(tower_positions, tower_colors_base):
        tower_height = 25
        tower_width = 4
        print(f"Building tower at ({tx}, {tz})...")

        for y in range(tower_height):
            for x in range(tower_width):
                for z in range(tower_width):
                    # Gradient from base color to white
                    t = y / tower_height
                    color = [min(1.0, c + t * 0.3) for c in base_color]
                    scene.add_cube_colored(
                        [tx + x * 1.05 - tower_width * 0.525 + 0.525,
                         y * 1.05 + 0.525,
                         tz + z * 1.05 - tower_width * 0.525 + 0.525],
                        0.5, 1.0, color
                    )
                    cube_count += 1

    print(f"  After towers: {cube_count} cubes")

    # Connecting walls between towers
    wall_segments = [
        ((-25, -25), (-25, 25)),  # Left side
        ((25, -25), (25, 25)),    # Right side
        ((-25, -25), (25, -25)),  # Front
        ((-25, 25), (25, 25)),    # Back
    ]

    wall_height = 10
    for (x1, z1), (x2, z2) in wall_segments:
        dx = 1 if x2 > x1 else (-1 if x2 < x1 else 0)
        dz = 1 if z2 > z1 else (-1 if z2 < z1 else 0)

        steps = max(abs(x2 - x1), abs(z2 - z1))
        if steps == 0:
            continue

        for i in range(int(steps)):
            wx = x1 + dx * i * 1.05
            wz = z1 + dz * i * 1.05

            for y in range(wall_height):
                # Stone gray with slight variation
                gray = 0.55 + 0.1 * math.sin(i * 0.5 + y * 0.3)
                color = [gray * 0.95, gray, gray * 0.9]
                scene.add_cube_colored(
                    [wx, y * 1.05 + 0.525, wz],
                    0.5, 1.2, color
                )
                cube_count += 1

    print(f"  After walls: {cube_count} cubes")

    # Fill gaps with smaller structures
    inner_positions = [
        (-12, 0), (12, 0), (0, -12), (0, 12),
        (-12, -12), (-12, 12), (12, -12), (12, 12)
    ]

    for px, pz in inner_positions:
        height = random.randint(6, 12)
        width = random.choice([2, 3])

        base_hue = random.random()
        for y in range(height):
            for x in range(width):
                for z in range(width):
                    # Varied colors per small building
                    hue = base_hue + y * 0.02
                    color = [
                        0.4 + 0.5 * math.sin(hue * 2 * math.pi),
                        0.4 + 0.5 * math.sin(hue * 2 * math.pi + 2.09),
                        0.4 + 0.5 * math.sin(hue * 2 * math.pi + 4.19),
                    ]
                    scene.add_cube_colored(
                        [px + x * 1.05 - width * 0.525 + 0.525,
                         y * 1.05 + 0.525,
                         pz + z * 1.05 - width * 0.525 + 0.525],
                        0.5, 1.0, color
                    )
                    cube_count += 1

    print(f"Total cubes: {cube_count}")

    # 50 balls in waves
    ball_configs = []

    # Wave 1: Horizontal barrage (20 balls)
    for i in range(20):
        angle = i * 2 * math.pi / 20
        distance = 55.0
        height = 3.0 + (i % 5) * 3.0

        start_x = math.cos(angle) * distance
        start_z = math.sin(angle) * distance

        speed = 45.0
        vel_x = -math.cos(angle) * speed
        vel_z = -math.sin(angle) * speed

        # Metallic colors
        color = [
            0.6 + 0.4 * math.sin(i * 0.5),
            0.6 + 0.4 * math.sin(i * 0.5 + 2.09),
            0.6 + 0.4 * math.sin(i * 0.5 + 4.19),
        ]

        ball_configs.append(([start_x, height, start_z], [vel_x, 2.0, vel_z], 1.3, 18.0, color))

    # Wave 2: Aerial bombardment (15 balls)
    for i in range(15):
        x = (i % 5 - 2) * 12
        z = (i // 5 - 1) * 12
        height = 45 + random.uniform(0, 15)

        color = [0.9, 0.5 + i * 0.03, 0.2]  # Orange gradient

        ball_configs.append(([x, height, z], [random.uniform(-5, 5), -30, random.uniform(-5, 5)], 1.8, 30.0, color))

    # Wave 3: Heavy hitters (10 balls)
    for i in range(10):
        angle = i * 2 * math.pi / 10 + math.pi / 20  # Offset from wave 1
        distance = 50.0
        height = 15.0 + i * 2

        start_x = math.cos(angle) * distance
        start_z = math.sin(angle) * distance

        speed = 50.0
        vel_x = -math.cos(angle) * speed
        vel_z = -math.sin(angle) * speed

        # Deep colors
        color = [0.3 + i * 0.05, 0.2, 0.7 - i * 0.05]

        ball_configs.append(([start_x, height, start_z], [vel_x, -3.0, vel_z], 2.0, 40.0, color))

    # Wave 4: Chaos balls (5 giant balls)
    for i in range(5):
        x = (i - 2) * 20
        y = 60
        z = random.uniform(-15, 15)

        color = [0.9, 0.2, 0.2 + i * 0.15]

        ball_configs.append(([x, y, z], [random.uniform(-10, 10), -35, random.uniform(-10, 10)], 2.5, 60.0, color))

    for pos, vel, radius, mass, color in ball_configs:
        scene.add_sphere_with_velocity_colored(pos, vel, radius, mass, color)

    cubes, spheres = scene.shape_counts()
    print(f"Scene 5: {cubes} cubes, {spheres} balls")

    sim = physobx.Simulator(scene, width=1920, height=1080)
    sim.set_camera([70.0, 45.0, 70.0], [0.0, 10.0, 0.0])

    fps = 60
    duration = 10.0
    total_frames = int(fps * duration)

    print(f"Rendering {total_frames} frames ({duration}s at {fps}fps)...")

    for frame in range(total_frames):
        sim.step(1.0 / fps)
        filename = f"{output_dir}/frame_{frame:04d}.png"
        sim.save_png(filename)
        if frame % 60 == 0:
            print(f"  Frame {frame}/{total_frames}")

    print(f"Done! Frames saved to {output_dir}/")


if __name__ == "__main__":
    main()
