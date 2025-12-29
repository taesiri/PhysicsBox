"""Scene 7: Domino Apocalypse - 8,000+ dominos in spiral patterns with chain reactions."""

import physobx
import os
import math
import random
from datetime import datetime

def main():
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    output_dir = f"./render/scene7_domino_apocalypse_{timestamp}"
    os.makedirs(output_dir, exist_ok=True)
    print(f"Output folder: {output_dir}")

    random.seed(777)

    scene = physobx.Scene()
    scene.add_ground(0.0, 400.0)

    cube_count = 0

    # Domino dimensions (tall and thin)
    domino_half_height = 1.0
    domino_half_width = 0.4
    domino_half_depth = 0.15

    print("Building spiral domino paths...")

    # Multiple spiraling paths from center
    num_spirals = 8
    dominos_per_spiral = 400

    for spiral in range(num_spirals):
        base_angle = spiral * 2 * math.pi / num_spirals
        # Spiral color
        hue = spiral / num_spirals
        base_color = [
            0.5 + 0.5 * math.sin(hue * 2 * math.pi),
            0.5 + 0.5 * math.sin(hue * 2 * math.pi + 2.09),
            0.5 + 0.5 * math.sin(hue * 2 * math.pi + 4.19),
        ]

        for i in range(dominos_per_spiral):
            # Spiral outward
            t = i / dominos_per_spiral
            radius = 5 + t * 60
            angle = base_angle + t * 6 * math.pi  # 3 full rotations

            x = math.cos(angle) * radius
            z = math.sin(angle) * radius

            # Domino faces tangent to spiral
            facing_angle = angle + math.pi / 2

            # Slight color variation
            variation = 0.1 * math.sin(i * 0.1)
            color = [max(0, min(1, c + variation)) for c in base_color]

            # Add domino (rotated to face tangent direction)
            # Using rotation quaternion for standing domino facing the right way
            qw = math.cos(facing_angle / 2)
            qy = math.sin(facing_angle / 2)

            scene.add_cube_colored([x, domino_half_height + 0.01, z], 0.5, 0.3, color)
            cube_count += 1

    print(f"  Spiral dominos: {cube_count}")

    # Concentric ring structures at various radii
    print("Building ring structures...")
    ring_start = cube_count
    ring_radii = [20, 35, 50, 65]

    for ring_idx, radius in enumerate(ring_radii):
        circumference = 2 * math.pi * radius
        num_dominos = int(circumference / 1.5)

        ring_hue = ring_idx / len(ring_radii)
        base_color = [
            0.9 - ring_idx * 0.15,
            0.3 + ring_idx * 0.15,
            0.2 + ring_idx * 0.2,
        ]

        for i in range(num_dominos):
            angle = i * 2 * math.pi / num_dominos
            x = math.cos(angle) * radius
            z = math.sin(angle) * radius

            color = [c * (0.9 + 0.2 * random.random()) for c in base_color]
            scene.add_cube_colored([x, domino_half_height + 0.01, z], 0.5, 0.3, color)
            cube_count += 1

    print(f"  Ring dominos: {cube_count - ring_start}")

    # Central tower that will collapse
    print("Building central mega-tower...")
    tower_start = cube_count
    tower_height = 30
    tower_base = 6

    for y in range(tower_height):
        # Tower narrows as it goes up
        size = max(2, tower_base - y // 6)
        for dx in range(size):
            for dz in range(size):
                t = y / tower_height
                # Gold to white gradient
                color = [0.9, 0.7 + t * 0.3, 0.2 + t * 0.7]
                scene.add_cube_colored(
                    [dx * 1.1 - size * 0.55 + 0.55,
                     y * 1.1 + 0.55,
                     dz * 1.1 - size * 0.55 + 0.55],
                    0.5, 1.0, color
                )
                cube_count += 1

    print(f"  Tower: {cube_count - tower_start}")

    # Radiating lines of dominos from tower
    print("Building radial lines...")
    radial_start = cube_count
    num_radials = 16

    for radial in range(num_radials):
        angle = radial * 2 * math.pi / num_radials
        # Skip angles that overlap with spiral starts
        hue = radial / num_radials

        for dist in range(8, 75, 2):
            x = math.cos(angle) * dist
            z = math.sin(angle) * dist

            color = [
                0.4 + 0.4 * math.sin(hue * 2 * math.pi + dist * 0.1),
                0.4 + 0.4 * math.sin(hue * 2 * math.pi + 2.09 + dist * 0.1),
                0.4 + 0.4 * math.sin(hue * 2 * math.pi + 4.19 + dist * 0.1),
            ]
            scene.add_cube_colored([x, domino_half_height + 0.01, z], 0.5, 0.3, color)
            cube_count += 1

    print(f"  Radial lines: {cube_count - radial_start}")

    # Corner monuments
    print("Building corner monuments...")
    monument_start = cube_count
    monument_positions = [(60, 60), (-60, 60), (60, -60), (-60, -60)]
    monument_colors = [
        [0.8, 0.2, 0.2],
        [0.2, 0.8, 0.2],
        [0.2, 0.2, 0.8],
        [0.8, 0.8, 0.2],
    ]

    for (mx, mz), base_color in zip(monument_positions, monument_colors):
        # Pyramid monument
        for level in range(10):
            size = 10 - level
            for dx in range(size):
                for dz in range(size):
                    t = level / 10
                    color = [c * (1 - t * 0.4) for c in base_color]
                    scene.add_cube_colored(
                        [mx + dx * 1.05 - size * 0.525 + 0.525,
                         level * 1.05 + 0.525,
                         mz + dz * 1.05 - size * 0.525 + 0.525],
                        0.5, 0.8, color
                    )
                    cube_count += 1

    print(f"  Monuments: {cube_count - monument_start}")
    print(f"Total cubes: {cube_count}")

    # Trigger balls
    print("Adding trigger balls...")

    # Central strike to topple tower
    scene.add_sphere_with_velocity_colored(
        [-30, 15, 0], [50, 0, 0], 2.5, 60, [0.9, 0.2, 0.2]
    )

    # Spiral starters (8 balls to start each spiral)
    for i in range(num_spirals):
        angle = i * 2 * math.pi / num_spirals
        x = math.cos(angle) * 8
        z = math.sin(angle) * 8

        # Push tangent to start domino chain
        push_angle = angle + math.pi / 2
        vel_x = math.cos(push_angle) * 15
        vel_z = math.sin(push_angle) * 15

        hue = i / num_spirals
        color = [
            0.6 + 0.4 * math.sin(hue * 2 * math.pi),
            0.6 + 0.4 * math.sin(hue * 2 * math.pi + 2.09),
            0.6 + 0.4 * math.sin(hue * 2 * math.pi + 4.19),
        ]
        scene.add_sphere_with_velocity_colored(
            [x - vel_x * 0.5, 1.5, z - vel_z * 0.5],
            [vel_x, 0, vel_z],
            1.0, 8, color
        )

    # Ring smashers (4 balls per ring)
    for radius in ring_radii:
        for i in range(4):
            angle = i * math.pi / 2 + math.pi / 4
            x = math.cos(angle) * (radius + 10)
            z = math.sin(angle) * (radius + 10)

            vel_x = -math.cos(angle) * 30
            vel_z = -math.sin(angle) * 30

            scene.add_sphere_with_velocity_colored(
                [x, 2, z], [vel_x, 0, vel_z],
                1.5, 15, [0.5, 0.5, 0.9]
            )

    # Monument destroyers
    for (mx, mz), color in zip(monument_positions, monument_colors):
        scene.add_sphere_with_velocity_colored(
            [mx + 25 * (1 if mx < 0 else -1), 8, mz],
            [-25 * (1 if mx < 0 else -1), 0, 0],
            2.0, 30, color
        )
        scene.add_sphere_with_velocity_colored(
            [mx, 8, mz + 25 * (1 if mz < 0 else -1)],
            [0, 0, -25 * (1 if mz < 0 else -1)],
            2.0, 30, color
        )

    # Aerial chaos (20 balls from above)
    for i in range(20):
        x = random.uniform(-70, 70)
        z = random.uniform(-70, 70)
        scene.add_sphere_with_velocity_colored(
            [x, 50 + random.uniform(0, 20), z],
            [random.uniform(-10, 10), -30, random.uniform(-10, 10)],
            1.5 + random.uniform(0, 1), 20 + random.uniform(0, 15),
            [random.uniform(0.5, 1), random.uniform(0.3, 0.7), random.uniform(0.2, 0.5)]
        )

    cubes, spheres = scene.shape_counts()
    print(f"Scene 7: {cubes} cubes, {spheres} balls")

    sim = physobx.Simulator(scene, width=1920, height=1080)
    sim.set_camera([110.0, 70.0, 110.0], [0.0, 10.0, 0.0])

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
