"""Scene 9: Cube Tsunami - 15,000+ cubes in a massive wave crashing into structures."""

import physobx
import os
import math
import random
from datetime import datetime

def main():
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    output_dir = f"./render/scene9_cube_tsunami_{timestamp}"
    os.makedirs(output_dir, exist_ok=True)
    print(f"Output folder: {output_dir}")

    random.seed(999)

    scene = physobx.Scene()
    scene.add_ground(0.0, 600.0)

    cube_count = 0

    # THE TSUNAMI WAVE - massive wall of cubes
    print("Building tsunami wave...")
    wave_width = 120
    wave_height = 35
    wave_depth = 25

    for x in range(int(wave_width / 1.1)):
        for y in range(wave_height):
            for z in range(wave_depth):
                # Wave shape - higher in middle, curved
                wave_x = (x - wave_width / 2.2) / (wave_width / 2.2)
                wave_profile = 1.0 - wave_x ** 2  # Parabolic
                max_y_at_x = int(wave_height * (0.5 + 0.5 * wave_profile))

                if y > max_y_at_x:
                    continue

                # Position with wave curl
                curl = 3 * math.sin(y / wave_height * math.pi) * (1 - z / wave_depth)
                actual_z = -80 + z * 1.1 - curl

                # Ocean colors - deep blue to foam white
                depth_factor = z / wave_depth
                height_factor = y / wave_height

                if height_factor > 0.8:  # Foam
                    color = [0.9, 0.95, 1.0]
                elif height_factor > 0.6:  # Light blue
                    color = [0.4, 0.7, 0.9]
                else:  # Deep blue
                    color = [0.1 + depth_factor * 0.2, 0.3 + depth_factor * 0.3, 0.6 + depth_factor * 0.3]

                scene.add_cube_colored(
                    [x * 1.1 - wave_width / 2, y * 1.1 + 0.55, actual_z],
                    0.5, 0.7, color
                )
                cube_count += 1

    print(f"  Tsunami wave: {cube_count} cubes")

    # COASTAL CITY - structures in path of tsunami
    print("Building coastal city...")
    city_start = cube_count

    # Seawall (will be destroyed)
    print("  Building seawall...")
    for x in range(-50, 51, 1):
        for y in range(8):
            for z in range(3):
                gray = 0.5 + 0.1 * ((x + y) % 3 - 1)
                color = [gray, gray * 0.95, gray * 0.9]
                scene.add_cube_colored([x * 1.05, y * 1.05 + 0.525, -30 + z * 1.05], 0.5, 2.0, color)
                cube_count += 1

    print(f"    Seawall: {cube_count - city_start} cubes")

    # Beach houses (first row)
    print("  Building beach houses...")
    house_start = cube_count
    for house_idx in range(-8, 9):
        base_x = house_idx * 12
        base_z = -15

        # Small house
        house_color = [
            0.8 + random.uniform(-0.1, 0.1),
            0.6 + random.uniform(-0.1, 0.1),
            0.4 + random.uniform(-0.1, 0.1)
        ]

        for y in range(6):
            for x in range(4):
                for z in range(4):
                    scene.add_cube_colored(
                        [base_x + x * 1.05 - 2.1, y * 1.05 + 0.525, base_z + z * 1.05],
                        0.5, 0.8, house_color
                    )
                    cube_count += 1

        # Roof
        for level in range(3):
            size = 4 - level
            for x in range(size):
                for z in range(size):
                    roof_color = [0.6, 0.3, 0.2]
                    scene.add_cube_colored(
                        [base_x + x * 1.05 - size * 0.525 + 0.525,
                         6 * 1.05 + level * 1.05 + 0.525,
                         base_z + z * 1.05 - size * 0.525 + 0.525 + 2],
                        0.5, 0.6, roof_color
                    )
                    cube_count += 1

    print(f"    Beach houses: {cube_count - house_start} cubes")

    # Mid-city buildings
    print("  Building mid-city...")
    midcity_start = cube_count

    building_configs = [
        (-40, 5, 15, [5, 5], [0.7, 0.75, 0.8]),
        (-25, 5, 20, [4, 6], [0.85, 0.8, 0.7]),
        (-10, 5, 25, [6, 4], [0.6, 0.65, 0.7]),
        (5, 5, 22, [5, 5], [0.8, 0.75, 0.7]),
        (20, 5, 18, [4, 5], [0.75, 0.7, 0.75]),
        (35, 5, 16, [5, 4], [0.7, 0.7, 0.75]),
        (-35, 20, 12, [3, 4], [0.65, 0.7, 0.75]),
        (0, 20, 28, [4, 4], [0.8, 0.8, 0.85]),
        (30, 20, 14, [4, 3], [0.7, 0.75, 0.8]),
    ]

    for base_x, base_z, height, (width, depth), base_color in building_configs:
        for y in range(height):
            for x in range(width):
                for z in range(depth):
                    t = y / height
                    color = [c * (1 - t * 0.2) for c in base_color]
                    scene.add_cube_colored(
                        [base_x + x * 1.05, y * 1.05 + 0.525, base_z + z * 1.05],
                        0.5, 1.0, color
                    )
                    cube_count += 1

    print(f"    Mid-city: {cube_count - midcity_start} cubes")

    # Tall downtown towers
    print("  Building downtown towers...")
    downtown_start = cube_count

    tower_configs = [
        (-20, 35, 40, [0.5, 0.6, 0.8]),
        (0, 35, 50, [0.8, 0.7, 0.5]),
        (20, 35, 45, [0.6, 0.7, 0.8]),
        (-35, 45, 35, [0.7, 0.7, 0.75]),
        (35, 45, 38, [0.75, 0.7, 0.7]),
    ]

    for base_x, base_z, height, base_color in tower_configs:
        for y in range(height):
            size = 4 if y < height * 0.8 else 3
            for x in range(size):
                for z in range(size):
                    t = y / height
                    color = [c * (1 - t * 0.15) for c in base_color]
                    scene.add_cube_colored(
                        [base_x + x * 1.05 - size * 0.525 + 0.525,
                         y * 1.05 + 0.525,
                         base_z + z * 1.05 - size * 0.525 + 0.525],
                        0.5, 1.2, color
                    )
                    cube_count += 1

    print(f"    Downtown: {cube_count - downtown_start} cubes")

    # Harbor structures
    print("  Building harbor...")
    harbor_start = cube_count

    # Pier
    for x in range(-30, 31):
        for z in range(-55, -35):
            if abs(x) < 5 or z > -40:
                scene.add_cube_colored([x * 1.05, 0.525, z * 1.05], 0.5, 1.5, [0.5, 0.4, 0.3])
                cube_count += 1

    # Cranes
    for crane_x in [-20, 20]:
        for y in range(20):
            scene.add_cube_colored([crane_x, y * 1.05 + 0.525, -50], 0.5, 0.8, [0.9, 0.7, 0.2])
            cube_count += 1
        for x in range(10):
            scene.add_cube_colored([crane_x + x * 1.05, 20 * 1.05 + 0.525, -50], 0.5, 0.8, [0.9, 0.7, 0.2])
            cube_count += 1

    print(f"    Harbor: {cube_count - harbor_start} cubes")
    print(f"Total cubes: {cube_count}")

    # PUSH BALLS - to get the tsunami moving fast
    print("Adding tsunami force balls...")

    # Massive pushers behind the wave
    for i in range(50):
        x = random.uniform(-55, 55)
        y = random.uniform(5, 30)
        z = -120

        scene.add_sphere_with_velocity_colored(
            [x, y, z],
            [0, 0, 80],  # Fast forward push
            2.5, 60,
            [0.2, 0.4, 0.8]
        )

    # Upper wave accelerators
    for i in range(30):
        x = random.uniform(-50, 50)
        y = random.uniform(20, 35)
        z = -100

        scene.add_sphere_with_velocity_colored(
            [x, y, z],
            [random.uniform(-5, 5), 10, 70],
            2.0, 40,
            [0.5, 0.7, 0.9]
        )

    # Side compression balls
    for i in range(20):
        side = 1 if i % 2 == 0 else -1
        x = side * 70
        y = random.uniform(5, 25)
        z = random.uniform(-100, -60)

        scene.add_sphere_with_velocity_colored(
            [x, y, z],
            [-side * 40, 5, 30],
            2.0, 35,
            [0.3, 0.5, 0.8]
        )

    # Aerial bombardment on the city (chaos after tsunami)
    for i in range(30):
        x = random.uniform(-40, 40)
        z = random.uniform(-10, 50)

        scene.add_sphere_with_velocity_colored(
            [x, 60 + random.uniform(0, 20), z],
            [random.uniform(-10, 10), -35, random.uniform(-15, 5)],
            1.5 + random.uniform(0, 1), 25,
            [0.8, 0.4, 0.2]
        )

    cubes, spheres = scene.shape_counts()
    print(f"Scene 9: {cubes} cubes, {spheres} balls")

    sim = physobx.Simulator(scene, width=1920, height=1080)
    sim.set_camera([100.0, 60.0, 80.0], [0.0, 15.0, -20.0])

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
