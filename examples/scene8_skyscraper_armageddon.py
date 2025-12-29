"""Scene 8: Skyscraper Armageddon - 12,000+ cubes in a city hit by meteor shower."""

import physobx
import os
import math
import random
from datetime import datetime

def main():
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    output_dir = f"./render/scene8_skyscraper_armageddon_{timestamp}"
    os.makedirs(output_dir, exist_ok=True)
    print(f"Output folder: {output_dir}")

    random.seed(2024)

    scene = physobx.Scene()
    scene.add_ground(0.0, 500.0)

    cube_count = 0

    # Building color palettes
    palettes = [
        [[0.7, 0.75, 0.8], [0.65, 0.7, 0.75]],      # Steel blue
        [[0.85, 0.82, 0.75], [0.8, 0.77, 0.7]],     # Sandstone
        [[0.6, 0.65, 0.7], [0.55, 0.6, 0.65]],      # Concrete
        [[0.75, 0.7, 0.65], [0.7, 0.65, 0.6]],      # Brown
        [[0.8, 0.8, 0.85], [0.75, 0.75, 0.8]],      # White
        [[0.5, 0.55, 0.6], [0.45, 0.5, 0.55]],      # Dark gray
        [[0.9, 0.85, 0.7], [0.85, 0.8, 0.65]],      # Cream
    ]

    # City grid
    grid_size = 9
    block_spacing = 18

    print("Building skyscraper district...")

    for gx in range(grid_size):
        for gz in range(grid_size):
            # Center area has tallest buildings
            dist_from_center = math.sqrt((gx - grid_size//2)**2 + (gz - grid_size//2)**2)

            # Skip some blocks for streets
            if random.random() < 0.15:
                continue

            base_x = (gx - grid_size // 2) * block_spacing
            base_z = (gz - grid_size // 2) * block_spacing

            # Building parameters vary by location
            if dist_from_center < 2:
                # Central skyscrapers - very tall
                height = random.randint(35, 50)
                width = random.randint(4, 6)
                depth = random.randint(4, 6)
            elif dist_from_center < 4:
                # Inner ring - tall
                height = random.randint(20, 35)
                width = random.randint(3, 5)
                depth = random.randint(3, 5)
            else:
                # Outer ring - shorter
                height = random.randint(8, 20)
                width = random.randint(2, 4)
                depth = random.randint(2, 4)

            palette = random.choice(palettes)
            building_start = cube_count

            # Main building
            for y in range(height):
                # Some buildings have setbacks
                current_width = width
                current_depth = depth
                if height > 30 and y > height * 0.7:
                    current_width = max(2, width - 1)
                    current_depth = max(2, depth - 1)

                for x in range(current_width):
                    for z in range(current_depth):
                        # Alternate colors by floor
                        color = palette[y % 2]
                        # Darken with height slightly
                        factor = 1.0 - (y / height) * 0.15
                        color = [c * factor for c in color]

                        scene.add_cube_colored(
                            [base_x + x * 1.05 - current_width * 0.525 + 0.525,
                             y * 1.05 + 0.525,
                             base_z + z * 1.05 - current_depth * 0.525 + 0.525],
                            0.5, 0.9, color
                        )
                        cube_count += 1

            # Add antenna/spire to tall buildings
            if height > 25:
                spire_height = random.randint(5, 10)
                for y in range(spire_height):
                    color = [0.6, 0.6, 0.65]
                    scene.add_cube_colored(
                        [base_x, height * 1.05 + y * 1.05 + 0.525, base_z],
                        0.3, 0.3, color
                    )
                    cube_count += 1

            if (gx * grid_size + gz) % 10 == 0:
                print(f"  Building at ({gx},{gz}): {cube_count - building_start} cubes, height {height}")

    print(f"Total buildings: {cube_count} cubes")

    # Central landmark tower (super tall)
    print("Building central landmark...")
    landmark_start = cube_count
    landmark_height = 60

    for y in range(landmark_height):
        # Tapering design
        if y < landmark_height * 0.6:
            size = 5
        elif y < landmark_height * 0.8:
            size = 4
        else:
            size = 3

        for x in range(size):
            for z in range(size):
                t = y / landmark_height
                # Gradient from blue to gold
                color = [
                    0.3 + t * 0.6,
                    0.4 + t * 0.4,
                    0.8 - t * 0.5
                ]
                scene.add_cube_colored(
                    [x * 1.05 - size * 0.525 + 0.525,
                     y * 1.05 + 0.525,
                     z * 1.05 - size * 0.525 + 0.525],
                    0.5, 1.5, color
                )
                cube_count += 1

    print(f"  Landmark: {cube_count - landmark_start} cubes")

    # Bridge structures connecting some buildings
    print("Building bridges...")
    bridge_start = cube_count
    bridge_positions = [
        ((-18, 15, -9), (0, 15, -9)),
        ((18, 12, 9), (36, 12, 9)),
        ((-9, 18, 18), (-9, 18, 36)),
        ((9, 20, -18), (9, 20, -36)),
    ]

    for (x1, y, z1), (x2, y2, z2) in bridge_positions:
        steps = int(max(abs(x2-x1), abs(z2-z1)) / 1.05)
        for i in range(steps):
            t = i / max(1, steps - 1)
            x = x1 + (x2 - x1) * t
            z = z1 + (z2 - z1) * t
            # Bridge sags slightly in middle
            sag = -2 * math.sin(t * math.pi)

            scene.add_cube_colored([x, y + sag, z], 0.5, 0.8, [0.5, 0.5, 0.55])
            cube_count += 1

    print(f"  Bridges: {cube_count - bridge_start} cubes")
    print(f"Total cubes: {cube_count}")

    # METEOR SHOWER - 150 balls
    print("Launching meteor shower...")

    # Main meteor swarm (100 meteors)
    for i in range(100):
        # Random position in a wide area above the city
        x = random.uniform(-80, 80)
        z = random.uniform(-80, 80)
        height = random.uniform(70, 120)

        # Meteors come from roughly the same direction (northwest)
        base_vel_x = 15 + random.uniform(-10, 10)
        base_vel_z = 15 + random.uniform(-10, 10)
        vel_y = -40 - random.uniform(0, 20)

        # Size varies
        radius = random.uniform(1.0, 2.5)
        mass = radius ** 3 * 10  # Mass proportional to volume

        # Fiery colors
        fire_t = random.random()
        color = [
            0.9,
            0.3 + fire_t * 0.5,
            0.1 + fire_t * 0.2
        ]

        scene.add_sphere_with_velocity_colored(
            [x, height, z],
            [base_vel_x, vel_y, base_vel_z],
            radius, mass, color
        )

    # Giant extinction-level meteors (10)
    for i in range(10):
        x = random.uniform(-40, 40)
        z = random.uniform(-40, 40)

        scene.add_sphere_with_velocity_colored(
            [x, 100 + i * 5, z],
            [random.uniform(-5, 20), -50, random.uniform(-5, 20)],
            4.0, 150, [1.0, 0.4, 0.1]
        )

    # Targeted strikes on landmark (5)
    for i in range(5):
        offset_x = random.uniform(-5, 5)
        offset_z = random.uniform(-5, 5)
        scene.add_sphere_with_velocity_colored(
            [offset_x + 30, 80 + i * 3, offset_z - 30],
            [-35, -35, 35],
            3.0, 80, [0.9, 0.5, 0.2]
        )

    # Side impacts (20)
    for i in range(20):
        angle = i * 2 * math.pi / 20
        distance = 90
        x = math.cos(angle) * distance
        z = math.sin(angle) * distance

        vel_x = -math.cos(angle) * 45
        vel_z = -math.sin(angle) * 45

        scene.add_sphere_with_velocity_colored(
            [x, 25 + random.uniform(0, 20), z],
            [vel_x, random.uniform(-10, 5), vel_z],
            2.0 + random.uniform(0, 1), 40 + random.uniform(0, 30),
            [0.8, 0.5, 0.3]
        )

    # Low altitude screamers (15)
    for i in range(15):
        x = random.uniform(-70, 70)
        z = -100
        height = random.uniform(10, 30)

        scene.add_sphere_with_velocity_colored(
            [x, height, z],
            [random.uniform(-10, 10), random.uniform(-5, 5), 60],
            1.8, 35, [0.7, 0.7, 0.3]
        )

    cubes, spheres = scene.shape_counts()
    print(f"Scene 8: {cubes} cubes, {spheres} meteors")

    sim = physobx.Simulator(scene, width=1920, height=1080)
    sim.set_camera([130.0, 80.0, 130.0], [0.0, 25.0, 0.0])

    fps = 60
    duration = 12.0
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
