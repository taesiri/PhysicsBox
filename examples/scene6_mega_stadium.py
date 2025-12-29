"""Scene 6: Mega Stadium Demolition - 10,000+ cubes stadium destroyed by 100 wrecking balls."""

import physobx
import os
import math
import random
from datetime import datetime

def main():
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    output_dir = f"./render/scene6_mega_stadium_{timestamp}"
    os.makedirs(output_dir, exist_ok=True)
    print(f"Output folder: {output_dir}")

    random.seed(42)

    scene = physobx.Scene()
    scene.add_ground(0.0, 400.0)

    cube_count = 0

    # Stadium parameters
    outer_radius = 50
    inner_radius = 30
    stadium_height = 25
    sections = 64

    print("Building stadium outer walls...")
    # Outer curved wall
    for section in range(sections):
        angle = section * 2 * math.pi / sections
        next_angle = (section + 1) * 2 * math.pi / sections

        x = math.cos(angle) * outer_radius
        z = math.sin(angle) * outer_radius

        # Color varies by section (rainbow around stadium)
        hue = section / sections
        color = [
            0.5 + 0.4 * math.sin(hue * 2 * math.pi),
            0.5 + 0.4 * math.sin(hue * 2 * math.pi + 2.09),
            0.5 + 0.4 * math.sin(hue * 2 * math.pi + 4.19),
        ]

        for y in range(stadium_height):
            # Taper height towards top
            height_factor = 1.0 - (y / stadium_height) * 0.3
            for layer in range(3):
                r = outer_radius - layer * 1.05
                lx = math.cos(angle) * r
                lz = math.sin(angle) * r

                # Darken with height
                layer_color = [c * (1.0 - y * 0.02) for c in color]
                scene.add_cube_colored([lx, y * 1.05 + 0.525, lz], 0.5, 0.8, layer_color)
                cube_count += 1

    print(f"  Outer walls: {cube_count} cubes")

    # Inner seating tiers (stepped)
    print("Building seating tiers...")
    tier_start = cube_count
    for tier in range(8):
        tier_radius = inner_radius + tier * 2.5
        tier_height = tier * 2
        tier_sections = int(sections * (tier_radius / outer_radius))

        for section in range(tier_sections):
            angle = section * 2 * math.pi / tier_sections
            x = math.cos(angle) * tier_radius
            z = math.sin(angle) * tier_radius

            # Seat colors - alternating team colors
            if (section // 4) % 2 == 0:
                color = [0.8, 0.2, 0.2]  # Red
            else:
                color = [0.2, 0.2, 0.8]  # Blue

            for h in range(2):
                scene.add_cube_colored([x, tier_height + h * 1.05 + 0.525, z], 0.5, 0.6, color)
                cube_count += 1

    print(f"  Seating: {cube_count - tier_start} cubes")

    # Four corner towers
    print("Building corner towers...")
    tower_start = cube_count
    tower_positions = [
        (outer_radius * 0.85, outer_radius * 0.85),
        (-outer_radius * 0.85, outer_radius * 0.85),
        (outer_radius * 0.85, -outer_radius * 0.85),
        (-outer_radius * 0.85, -outer_radius * 0.85),
    ]
    tower_colors = [
        [0.9, 0.7, 0.2],  # Gold
        [0.7, 0.7, 0.8],  # Silver
        [0.8, 0.5, 0.2],  # Bronze
        [0.6, 0.9, 0.6],  # Green
    ]

    for (tx, tz), base_color in zip(tower_positions, tower_colors):
        tower_height = 40
        for y in range(tower_height):
            # Tower narrows slightly
            size = 3 if y < tower_height * 0.7 else 2
            for dx in range(size):
                for dz in range(size):
                    t = y / tower_height
                    color = [c * (1.0 - t * 0.3) + t * 0.3 for c in base_color]
                    scene.add_cube_colored(
                        [tx + dx * 1.05 - size * 0.525 + 0.525,
                         y * 1.05 + 0.525,
                         tz + dz * 1.05 - size * 0.525 + 0.525],
                        0.5, 1.2, color
                    )
                    cube_count += 1

    print(f"  Towers: {cube_count - tower_start} cubes")

    # Central field structure
    print("Building field structures...")
    field_start = cube_count
    # Goal posts at each end
    for end in [-1, 1]:
        post_z = end * 20
        for post_x in [-5, 5]:
            for y in range(12):
                color = [0.9, 0.9, 0.9]  # White
                scene.add_cube_colored([post_x, y * 1.05 + 0.525, post_z], 0.4, 0.5, color)
                cube_count += 1
        # Crossbar
        for x in range(-4, 5):
            scene.add_cube_colored([x * 1.05, 12 * 1.05 + 0.525, post_z], 0.4, 0.5, [0.9, 0.9, 0.9])
            cube_count += 1

    # Center monument
    for y in range(15):
        size = max(1, 4 - y // 4)
        for dx in range(size):
            for dz in range(size):
                t = y / 15
                color = [0.9 - t * 0.3, 0.7 - t * 0.2, 0.2 + t * 0.5]
                scene.add_cube_colored(
                    [dx * 1.05 - size * 0.525 + 0.525,
                     y * 1.05 + 0.525,
                     dz * 1.05 - size * 0.525 + 0.525],
                    0.5, 1.5, color
                )
                cube_count += 1

    print(f"  Field: {cube_count - field_start} cubes")
    print(f"Total cubes: {cube_count}")

    # 100 wrecking balls from all directions
    print("Adding wrecking balls...")

    # Wave 1: Ring attack (40 balls)
    for i in range(40):
        angle = i * 2 * math.pi / 40
        distance = 80
        height = 10 + (i % 5) * 8

        x = math.cos(angle) * distance
        z = math.sin(angle) * distance

        speed = 50 + random.uniform(-5, 10)
        vel_x = -math.cos(angle) * speed
        vel_z = -math.sin(angle) * speed

        hue = i / 40
        color = [
            0.6 + 0.4 * math.sin(hue * 4 * math.pi),
            0.6 + 0.4 * math.sin(hue * 4 * math.pi + 2.09),
            0.6 + 0.4 * math.sin(hue * 4 * math.pi + 4.19),
        ]

        radius = 1.5 + random.uniform(0, 1.0)
        mass = 25 + random.uniform(0, 20)

        scene.add_sphere_with_velocity_colored(
            [x, height, z], [vel_x, random.uniform(-3, 5), vel_z],
            radius, mass, color
        )

    # Wave 2: Aerial bombardment (35 balls)
    for i in range(35):
        x = (i % 7 - 3) * 15
        z = (i // 7 - 2.5) * 15
        height = 60 + random.uniform(0, 20)

        color = [0.9, 0.4 + i * 0.015, 0.2]
        radius = 2.0 + random.uniform(0, 1.0)
        mass = 35 + random.uniform(0, 25)

        scene.add_sphere_with_velocity_colored(
            [x, height, z],
            [random.uniform(-8, 8), -35 - random.uniform(0, 15), random.uniform(-8, 8)],
            radius, mass, color
        )

    # Wave 3: Tower destroyers (16 balls targeting corners)
    for i, (tx, tz) in enumerate(tower_positions):
        for j in range(4):
            angle = random.uniform(0, 2 * math.pi)
            dist = 70
            x = tx + math.cos(angle) * dist
            z = tz + math.sin(angle) * dist

            dx = tx - x
            dz = tz - z
            length = math.sqrt(dx*dx + dz*dz)
            speed = 55

            color = tower_colors[i]
            scene.add_sphere_with_velocity_colored(
                [x, 25 + j * 5, z],
                [dx/length * speed, -5, dz/length * speed],
                2.5, 50, color
            )

    # Wave 4: Giant meteor (9 massive balls)
    for i in range(9):
        x = (i % 3 - 1) * 25
        z = (i // 3 - 1) * 25

        color = [0.9, 0.3, 0.1]  # Fiery
        scene.add_sphere_with_velocity_colored(
            [x, 80, z],
            [random.uniform(-5, 5), -45, random.uniform(-5, 5)],
            3.5, 80, color
        )

    cubes, spheres = scene.shape_counts()
    print(f"Scene 6: {cubes} cubes, {spheres} balls")

    sim = physobx.Simulator(scene, width=1920, height=1080)
    sim.set_camera([100.0, 60.0, 100.0], [0.0, 15.0, 0.0])

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
