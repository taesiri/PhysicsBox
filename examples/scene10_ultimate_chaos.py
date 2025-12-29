"""Scene 10: Ultimate Chaos - 20,000+ cubes in the most extreme destruction scene."""

import physobx
import os
import math
import random
from datetime import datetime

def main():
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    output_dir = f"./render/scene10_ultimate_chaos_{timestamp}"
    os.makedirs(output_dir, exist_ok=True)
    print(f"Output folder: {output_dir}")

    random.seed(12345)

    scene = physobx.Scene()
    scene.add_ground(0.0, 800.0)

    cube_count = 0

    # ===== CENTRAL MEGA TOWER (The Spire) =====
    print("Building The Spire (central mega tower)...")
    spire_height = 80
    spire_base = 8

    for y in range(spire_height):
        # Tapering spire
        t = y / spire_height
        if t < 0.5:
            size = spire_base
        elif t < 0.7:
            size = spire_base - 1
        elif t < 0.85:
            size = spire_base - 2
        else:
            size = max(2, spire_base - 3)

        for x in range(size):
            for z in range(size):
                # Rainbow gradient up the spire
                hue = t
                color = [
                    0.5 + 0.5 * math.sin(hue * 4 * math.pi),
                    0.5 + 0.5 * math.sin(hue * 4 * math.pi + 2.09),
                    0.5 + 0.5 * math.sin(hue * 4 * math.pi + 4.19),
                ]
                scene.add_cube_colored(
                    [x * 1.05 - size * 0.525 + 0.525,
                     y * 1.05 + 0.525,
                     z * 1.05 - size * 0.525 + 0.525],
                    0.5, 1.5, color
                )
                cube_count += 1

    print(f"  The Spire: {cube_count} cubes")

    # ===== FOUR FORTRESS CORNERS =====
    print("Building corner fortresses...")
    fortress_start = cube_count
    fortress_positions = [(-60, -60), (-60, 60), (60, -60), (60, 60)]
    fortress_colors = [
        [0.8, 0.2, 0.2],  # Red
        [0.2, 0.8, 0.2],  # Green
        [0.2, 0.2, 0.8],  # Blue
        [0.8, 0.8, 0.2],  # Yellow
    ]

    for (fx, fz), base_color in zip(fortress_positions, fortress_colors):
        # Main keep
        for y in range(30):
            size = 6 if y < 20 else 5
            for x in range(size):
                for z in range(size):
                    t = y / 30
                    color = [c * (1 - t * 0.3) for c in base_color]
                    scene.add_cube_colored(
                        [fx + x * 1.05 - size * 0.525 + 0.525,
                         y * 1.05 + 0.525,
                         fz + z * 1.05 - size * 0.525 + 0.525],
                        0.5, 1.2, color
                    )
                    cube_count += 1

        # Corner turrets
        for (tx, tz) in [(-4, -4), (-4, 4), (4, -4), (4, 4)]:
            for y in range(20):
                for dx in range(2):
                    for dz in range(2):
                        t = y / 20
                        color = [c * (0.8 - t * 0.2) for c in base_color]
                        scene.add_cube_colored(
                            [fx + tx + dx * 1.05,
                             y * 1.05 + 0.525,
                             fz + tz + dz * 1.05],
                            0.5, 1.0, color
                        )
                        cube_count += 1

    print(f"  Fortresses: {cube_count - fortress_start} cubes")

    # ===== CONNECTING WALLS =====
    print("Building mega walls...")
    wall_start = cube_count
    wall_height = 15

    # Walls between fortresses
    wall_segments = [
        ((-60, -60), (-60, 60)),
        ((-60, 60), (60, 60)),
        ((60, 60), (60, -60)),
        ((60, -60), (-60, -60)),
    ]

    for (x1, z1), (x2, z2) in wall_segments:
        steps = int(max(abs(x2 - x1), abs(z2 - z1)) / 1.05)
        for i in range(steps):
            t = i / max(1, steps)
            x = x1 + (x2 - x1) * t
            z = z1 + (z2 - z1) * t

            for y in range(wall_height):
                for thickness in range(2):
                    # Stone wall
                    gray = 0.5 + 0.1 * math.sin(i * 0.3 + y * 0.5)
                    color = [gray, gray * 0.95, gray * 0.9]

                    if abs(x2 - x1) > abs(z2 - z1):
                        scene.add_cube_colored([x, y * 1.05 + 0.525, z + thickness * 1.05], 0.5, 1.5, color)
                    else:
                        scene.add_cube_colored([x + thickness * 1.05, y * 1.05 + 0.525, z], 0.5, 1.5, color)
                    cube_count += 1

    print(f"  Walls: {cube_count - wall_start} cubes")

    # ===== INNER CITY GRID =====
    print("Building inner city...")
    city_start = cube_count

    for bx in range(-4, 5):
        for bz in range(-4, 5):
            # Skip center (for spire) and very corners
            if abs(bx) <= 1 and abs(bz) <= 1:
                continue
            if abs(bx) == 4 and abs(bz) == 4:
                continue

            base_x = bx * 12
            base_z = bz * 12

            # Building height varies by distance from center
            dist = math.sqrt(bx*bx + bz*bz)
            height = random.randint(8, int(25 - dist * 2))
            width = random.randint(2, 4)
            depth = random.randint(2, 4)

            # Color based on position
            hue = (bx + bz + 8) / 16
            base_color = [
                0.5 + 0.3 * math.sin(hue * 2 * math.pi),
                0.5 + 0.3 * math.sin(hue * 2 * math.pi + 2.09),
                0.5 + 0.3 * math.sin(hue * 2 * math.pi + 4.19),
            ]

            for y in range(height):
                for x in range(width):
                    for z in range(depth):
                        t = y / height
                        color = [c * (1 - t * 0.2) for c in base_color]
                        scene.add_cube_colored(
                            [base_x + x * 1.05 - width * 0.525 + 0.525,
                             y * 1.05 + 0.525,
                             base_z + z * 1.05 - depth * 0.525 + 0.525],
                            0.5, 0.9, color
                        )
                        cube_count += 1

    print(f"  Inner city: {cube_count - city_start} cubes")

    # ===== RING MONUMENTS =====
    print("Building ring monuments...")
    monument_start = cube_count

    for ring_idx, ring_radius in enumerate([25, 40]):
        num_monuments = 8 if ring_idx == 0 else 12
        for i in range(num_monuments):
            angle = i * 2 * math.pi / num_monuments
            mx = math.cos(angle) * ring_radius
            mz = math.sin(angle) * ring_radius

            # Obelisk
            height = 20 - ring_idx * 5
            for y in range(height):
                size = 2 if y < height - 3 else 1

                hue = i / num_monuments
                color = [
                    0.6 + 0.4 * math.sin(hue * 2 * math.pi + y * 0.2),
                    0.6 + 0.4 * math.sin(hue * 2 * math.pi + 2.09 + y * 0.2),
                    0.6 + 0.4 * math.sin(hue * 2 * math.pi + 4.19 + y * 0.2),
                ]

                for dx in range(size):
                    for dz in range(size):
                        scene.add_cube_colored(
                            [mx + dx * 1.05 - size * 0.525 + 0.525,
                             y * 1.05 + 0.525,
                             mz + dz * 1.05 - size * 0.525 + 0.525],
                            0.5, 1.0, color
                        )
                        cube_count += 1

    print(f"  Monuments: {cube_count - monument_start} cubes")

    # ===== FLOATING PLATFORMS (will fall) =====
    print("Building floating platforms...")
    platform_start = cube_count

    platform_heights = [50, 55, 60, 65, 70]
    for ph in platform_heights:
        platform_size = 30 - (ph - 50)
        for x in range(platform_size):
            for z in range(platform_size):
                # Skip middle
                dist = math.sqrt((x - platform_size/2)**2 + (z - platform_size/2)**2)
                if dist < 3:
                    continue

                hue = ph / 80
                color = [
                    0.4 + 0.4 * math.sin(hue * 3 * math.pi),
                    0.4 + 0.4 * math.sin(hue * 3 * math.pi + 2.09),
                    0.4 + 0.4 * math.sin(hue * 3 * math.pi + 4.19),
                ]
                scene.add_cube_colored(
                    [x * 1.1 - platform_size * 0.55 + 0.55,
                     ph,
                     z * 1.1 - platform_size * 0.55 + 0.55],
                    0.5, 0.6, color
                )
                cube_count += 1

    print(f"  Platforms: {cube_count - platform_start} cubes")
    print(f"Total cubes: {cube_count}")

    # ===== ULTIMATE BALL ASSAULT - 200 BALLS =====
    print("Launching ultimate assault...")

    # Wave 1: Outer ring attack (50 balls)
    for i in range(50):
        angle = i * 2 * math.pi / 50
        distance = 120
        height = 20 + (i % 5) * 10

        x = math.cos(angle) * distance
        z = math.sin(angle) * distance

        speed = 60
        vel_x = -math.cos(angle) * speed
        vel_z = -math.sin(angle) * speed

        hue = i / 50
        color = [
            0.5 + 0.5 * math.sin(hue * 4 * math.pi),
            0.5 + 0.5 * math.sin(hue * 4 * math.pi + 2.09),
            0.5 + 0.5 * math.sin(hue * 4 * math.pi + 4.19),
        ]

        scene.add_sphere_with_velocity_colored(
            [x, height, z], [vel_x, random.uniform(-5, 10), vel_z],
            2.0 + random.uniform(0, 1), 35 + random.uniform(0, 25), color
        )

    # Wave 2: Aerial bombardment (60 balls)
    for i in range(60):
        x = random.uniform(-70, 70)
        z = random.uniform(-70, 70)
        height = random.uniform(100, 150)

        color = [
            0.8 + random.uniform(0, 0.2),
            0.3 + random.uniform(0, 0.4),
            0.1 + random.uniform(0, 0.2)
        ]

        scene.add_sphere_with_velocity_colored(
            [x, height, z],
            [random.uniform(-10, 10), -50 - random.uniform(0, 20), random.uniform(-10, 10)],
            2.5 + random.uniform(0, 1.5), 50 + random.uniform(0, 40), color
        )

    # Wave 3: Fortress attackers (40 balls, 10 per fortress)
    for (fx, fz), fcolor in zip(fortress_positions, fortress_colors):
        for j in range(10):
            angle = random.uniform(0, 2 * math.pi)
            dist = 40
            x = fx + math.cos(angle) * dist
            z = fz + math.sin(angle) * dist

            dx = fx - x
            dz = fz - z
            length = math.sqrt(dx*dx + dz*dz)
            speed = 55

            scene.add_sphere_with_velocity_colored(
                [x, 15 + j * 3, z],
                [dx/length * speed, random.uniform(-5, 5), dz/length * speed],
                2.0 + random.uniform(0, 1), 40 + random.uniform(0, 20), fcolor
            )

    # Wave 4: Spire destroyers (20 balls)
    for i in range(20):
        angle = i * 2 * math.pi / 20
        dist = 50
        height = 30 + i * 2

        x = math.cos(angle) * dist
        z = math.sin(angle) * dist

        scene.add_sphere_with_velocity_colored(
            [x, height, z],
            [-x / dist * 50, 10, -z / dist * 50],
            2.5, 55, [0.9, 0.7, 0.2]
        )

    # Wave 5: Platform destroyers (15 balls)
    for i in range(15):
        scene.add_sphere_with_velocity_colored(
            [random.uniform(-20, 20), 90, random.uniform(-20, 20)],
            [random.uniform(-15, 15), -20, random.uniform(-15, 15)],
            3.0, 70, [0.6, 0.2, 0.8]
        )

    # Wave 6: Mega chaos balls (15 giant balls)
    for i in range(15):
        x = random.uniform(-80, 80)
        z = random.uniform(-80, 80)

        scene.add_sphere_with_velocity_colored(
            [x, 120 + i * 5, z],
            [random.uniform(-20, 20), -60, random.uniform(-20, 20)],
            4.0, 100, [1.0, 0.3, 0.1]
        )

    cubes, spheres = scene.shape_counts()
    print(f"Scene 10: {cubes} cubes, {spheres} balls")

    sim = physobx.Simulator(scene, width=1920, height=1080)
    sim.set_camera([150.0, 100.0, 150.0], [0.0, 30.0, 0.0])

    fps = 60
    duration = 15.0  # Longer duration for more chaos
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
