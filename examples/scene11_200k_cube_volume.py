"""Scene 11: 200K Cube Volume - A massive solid block of 200,000+ cubes."""

import physobx
import os
import math
from datetime import datetime

def main():
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    output_dir = f"./render/scene11_200k_cube_volume_{timestamp}"
    os.makedirs(output_dir, exist_ok=True)
    print(f"Output folder: {output_dir}")

    scene = physobx.Scene()
    scene.add_ground(0.0, 500.0)

    cube_count = 0

    # 200K cube volume: 59 x 59 x 58 = 201,898 cubes
    size_x = 59
    size_y = 58
    size_z = 59

    print(f"Building {size_x}x{size_y}x{size_z} cube volume ({size_x * size_y * size_z} cubes)...")

    cube_spacing = 1.05  # Slight gap for physics stability
    half_extent = 0.5

    # Center the volume
    offset_x = -size_x * cube_spacing / 2
    offset_z = -size_z * cube_spacing / 2
    start_y = 5.0  # Start slightly above ground

    for y in range(size_y):
        # Progress indicator
        if y % 10 == 0:
            print(f"  Layer {y}/{size_y}...")

        # Color gradient: bottom is dark blue, top is bright orange/yellow
        t = y / size_y

        for x in range(size_x):
            for z in range(size_z):
                # Calculate position
                px = offset_x + x * cube_spacing + half_extent
                py = start_y + y * cube_spacing + half_extent
                pz = offset_z + z * cube_spacing + half_extent

                # Gradient color based on height
                # Blue at bottom -> Cyan -> Green -> Yellow -> Orange at top
                if t < 0.25:
                    # Blue to cyan
                    local_t = t / 0.25
                    color = [0.1, 0.2 + local_t * 0.5, 0.8]
                elif t < 0.5:
                    # Cyan to green
                    local_t = (t - 0.25) / 0.25
                    color = [0.1, 0.7, 0.8 - local_t * 0.6]
                elif t < 0.75:
                    # Green to yellow
                    local_t = (t - 0.5) / 0.25
                    color = [0.1 + local_t * 0.8, 0.7, 0.2]
                else:
                    # Yellow to orange/red
                    local_t = (t - 0.75) / 0.25
                    color = [0.9, 0.7 - local_t * 0.4, 0.2 - local_t * 0.1]

                # Add slight variation based on position
                variation = 0.05 * math.sin(x * 0.5 + z * 0.3)
                color = [max(0, min(1, c + variation)) for c in color]

                scene.add_cube_colored([px, py, pz], half_extent, 0.8, color)
                cube_count += 1

    print(f"Total cubes: {cube_count}")

    # Add a few massive impact balls to break up the volume
    print("Adding impact balls...")

    # Giant ball from above center
    scene.add_sphere_with_velocity_colored(
        [0, 100, 0],
        [0, -80, 0],
        8.0, 500,
        [1.0, 0.2, 0.1]
    )

    # Side impacts
    for i in range(8):
        angle = i * 2 * math.pi / 8
        dist = 80
        x = math.cos(angle) * dist
        z = math.sin(angle) * dist
        height = 30 + (i % 3) * 10

        speed = 70
        vel_x = -math.cos(angle) * speed
        vel_z = -math.sin(angle) * speed

        hue = i / 8
        color = [
            0.6 + 0.4 * math.sin(hue * 2 * math.pi),
            0.6 + 0.4 * math.sin(hue * 2 * math.pi + 2.09),
            0.6 + 0.4 * math.sin(hue * 2 * math.pi + 4.19),
        ]

        scene.add_sphere_with_velocity_colored(
            [x, height, z],
            [vel_x, 0, vel_z],
            5.0, 200,
            color
        )

    cubes, spheres = scene.shape_counts()
    print(f"Scene 11: {cubes} cubes, {spheres} balls")

    sim = physobx.Simulator(scene, width=1920, height=1080)
    # Pull camera back to see the massive volume
    sim.set_camera([180.0, 100.0, 180.0], [0.0, 30.0, 0.0])

    fps = 60
    duration = 15.0
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
