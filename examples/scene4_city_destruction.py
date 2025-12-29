"""Scene 4: City Destruction - 2000+ cubes as buildings destroyed by 25 balls."""

import physobx
import os
import math
import random
from datetime import datetime

def main():
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    output_dir = f"./render/scene4_city_destruction_{timestamp}"
    os.makedirs(output_dir, exist_ok=True)
    print(f"Output folder: {output_dir}")

    random.seed(42)  # Reproducible randomness

    scene = physobx.Scene()
    scene.add_ground(0.0, 250.0)

    cube_count = 0

    # Building color palettes
    building_palettes = [
        [[0.85, 0.75, 0.65], [0.80, 0.70, 0.60]],  # Beige
        [[0.70, 0.70, 0.75], [0.65, 0.65, 0.70]],  # Gray-blue
        [[0.75, 0.65, 0.60], [0.70, 0.60, 0.55]],  # Brown
        [[0.65, 0.75, 0.70], [0.60, 0.70, 0.65]],  # Green-gray
        [[0.80, 0.70, 0.65], [0.75, 0.65, 0.60]],  # Tan
    ]

    # Create a grid of buildings (7x7 grid with gaps)
    grid_size = 7
    building_spacing = 8.0

    for gx in range(grid_size):
        for gz in range(grid_size):
            # Skip some positions for variety
            if (gx + gz) % 4 == 0 and gx != 3 and gz != 3:
                continue

            base_x = (gx - grid_size // 2) * building_spacing
            base_z = (gz - grid_size // 2) * building_spacing

            # Random building height (5-15)
            height = random.randint(5, 15)
            # Random building footprint (2x2 or 3x3)
            size = random.choice([2, 3])

            # Pick a palette
            palette = random.choice(building_palettes)

            for y in range(height):
                for x in range(size):
                    for z in range(size):
                        # Alternate colors by floor
                        color = palette[y % 2]
                        # Darken slightly with height
                        factor = 1.0 - (y / height) * 0.2
                        color = [c * factor for c in color]

                        scene.add_cube_colored(
                            [base_x + x * 1.05 - size * 0.525 + 0.525,
                             y * 1.05 + 0.525,
                             base_z + z * 1.05 - size * 0.525 + 0.525],
                            0.5, 1.0, color
                        )
                        cube_count += 1

    # Add a central tall tower
    tower_height = 20
    for y in range(tower_height):
        for x in range(4):
            for z in range(4):
                t = y / tower_height
                # Golden tower
                color = [0.9 - t * 0.2, 0.7 - t * 0.1, 0.3 + t * 0.2]
                scene.add_cube_colored(
                    [x * 1.05 - 2.1, y * 1.05 + 0.525, z * 1.05 - 2.1],
                    0.5, 2.0, color
                )
                cube_count += 1

    print(f"Built city with {cube_count} cubes")

    # 25 balls of various sizes from all directions
    ball_count = 0

    # Perimeter bombardment (16 balls)
    for i in range(16):
        angle = i * 2 * math.pi / 16
        distance = 45.0
        height = 5.0 + (i % 4) * 4.0

        start_x = math.cos(angle) * distance
        start_z = math.sin(angle) * distance

        speed = 40.0 + random.uniform(-5, 5)
        vel_x = -math.cos(angle) * speed
        vel_z = -math.sin(angle) * speed
        vel_y = random.uniform(-2, 3)

        # Colorful balls
        hue = i / 16
        color = [
            0.5 + 0.5 * math.sin(hue * 2 * math.pi),
            0.5 + 0.5 * math.sin(hue * 2 * math.pi + 2.09),
            0.5 + 0.5 * math.sin(hue * 2 * math.pi + 4.19),
        ]

        radius = 1.2 + random.uniform(0, 0.5)
        mass = 15.0 + random.uniform(0, 10)

        scene.add_sphere_with_velocity_colored(
            [start_x, height, start_z],
            [vel_x, vel_y, vel_z],
            radius, mass, color
        )
        ball_count += 1

    # Aerial bombardment (9 balls)
    for i in range(9):
        # 3x3 grid from above
        x = (i % 3 - 1) * 10
        z = (i // 3 - 1) * 10
        height = 40 + random.uniform(0, 10)

        color = [0.9, 0.3 + i * 0.07, 0.3]  # Red gradient
        radius = 1.5 + random.uniform(0, 0.5)
        mass = 25.0 + random.uniform(0, 15)

        scene.add_sphere_with_velocity_colored(
            [x, height, z],
            [random.uniform(-3, 3), -25, random.uniform(-3, 3)],
            radius, mass, color
        )
        ball_count += 1

    cubes, spheres = scene.shape_counts()
    print(f"Scene 4: {cubes} cubes, {spheres} balls")

    sim = physobx.Simulator(scene, width=1920, height=1080)
    sim.set_camera([55.0, 35.0, 55.0], [0.0, 8.0, 0.0])

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
