"""Instagram 4K Portrait Test - 5 scenes at 2160x3840 (9:16) for 10 seconds each."""

import physobx
import os
import math
import random
import subprocess
from datetime import datetime


def scene_1_tower_collapse(output_dir):
    """Massive tower collapse - tall narrow structure perfect for portrait."""
    print("\n=== Scene 1: Tower Collapse ===")

    scene = physobx.Scene()
    scene.add_ground(0.0, 100.0)

    cube_count = 0
    tower_height = 80
    tower_width = 6

    # Build a tall tower with rainbow gradient
    for y in range(tower_height):
        # Slight taper
        width = max(3, tower_width - y // 20)
        offset = (tower_width - width) / 2

        for x in range(width):
            for z in range(width):
                # Rainbow color based on height
                hue = y / tower_height
                color = [
                    0.5 + 0.5 * math.sin(hue * 2 * math.pi),
                    0.5 + 0.5 * math.sin(hue * 2 * math.pi + 2.09),
                    0.5 + 0.5 * math.sin(hue * 2 * math.pi + 4.19),
                ]
                scene.add_cube_colored(
                    [(x + offset) * 1.05 - tower_width * 0.525 + 0.525,
                     y * 1.05 + 0.525,
                     (z + offset) * 1.05 - tower_width * 0.525 + 0.525],
                    0.5, 1.0, color
                )
                cube_count += 1

    # Wrecking balls from the sides
    for i in range(15):
        angle = i * 2 * math.pi / 15
        height = 20 + (i % 5) * 12
        distance = 40

        x = math.cos(angle) * distance
        z = math.sin(angle) * distance

        speed = 45
        color = [0.8, 0.3 + i * 0.03, 0.2]

        scene.add_sphere_with_velocity_colored(
            [x, height, z],
            [-math.cos(angle) * speed, 0, -math.sin(angle) * speed],
            2.0, 40, color
        )

    # Top bombers
    for i in range(8):
        x = (i % 3 - 1) * 3
        z = (i // 3 - 1) * 3
        scene.add_sphere_with_velocity_colored(
            [x, 100, z], [0, -50, 0], 2.5, 60, [0.9, 0.5, 0.1]
        )

    cubes, spheres = scene.shape_counts()
    print(f"  {cubes} cubes, {spheres} balls")

    sim = physobx.Simulator(scene, width=2160, height=3840)
    sim.set_camera([35.0, 45.0, 35.0], [0.0, 35.0, 0.0])

    return sim, output_dir + "/scene1_tower"


def scene_2_domino_cascade(output_dir):
    """Spiral domino cascade - great vertical motion."""
    print("\n=== Scene 2: Spiral Domino Cascade ===")

    scene = physobx.Scene()
    scene.add_ground(0.0, 150.0)

    cube_count = 0

    # Spiral of tall dominoes going upward
    num_dominoes = 400
    spiral_turns = 5
    max_height = 60

    for i in range(num_dominoes):
        t = i / num_dominoes
        angle = t * spiral_turns * 2 * math.pi
        radius = 8 + t * 25
        height = t * max_height

        x = math.cos(angle) * radius
        z = math.sin(angle) * radius

        # Domino orientation - tangent to spiral
        rot_angle = angle + math.pi / 2

        # Color gradient
        hue = t
        color = [
            0.4 + 0.5 * math.sin(hue * 4 * math.pi),
            0.5 + 0.4 * math.sin(hue * 4 * math.pi + 2.09),
            0.6 + 0.3 * math.sin(hue * 4 * math.pi + 4.19),
        ]

        # Stack of cubes for each domino (3 high)
        for dy in range(4):
            scene.add_cube_colored(
                [x, height + dy * 1.05 + 0.525, z],
                0.5, 0.8, color
            )
            cube_count += 1

    # Trigger ball at the start
    start_angle = 0
    start_radius = 6
    scene.add_sphere_with_velocity_colored(
        [math.cos(start_angle) * start_radius, 5, math.sin(start_angle) * start_radius],
        [15, 0, 0], 3.0, 80, [1.0, 0.3, 0.1]
    )

    # Aerial balls
    for i in range(12):
        angle = i * 2 * math.pi / 12
        r = 20 + (i % 3) * 8
        h = 70 + (i % 4) * 10
        scene.add_sphere_with_velocity_colored(
            [math.cos(angle) * r, h, math.sin(angle) * r],
            [-math.cos(angle) * 30, -25, -math.sin(angle) * 30],
            2.0, 35, [0.2 + i * 0.05, 0.6, 0.9]
        )

    cubes, spheres = scene.shape_counts()
    print(f"  {cubes} cubes, {spheres} balls")

    sim = physobx.Simulator(scene, width=2160, height=3840)
    sim.set_camera([60.0, 50.0, 60.0], [0.0, 30.0, 0.0])

    return sim, output_dir + "/scene2_domino"


def scene_3_cube_waterfall(output_dir):
    """Cubes falling like a waterfall - perfect vertical composition."""
    print("\n=== Scene 3: Cube Waterfall ===")

    scene = physobx.Scene()
    scene.add_ground(0.0, 100.0)

    cube_count = 0

    # Stepped platforms creating waterfall effect
    num_steps = 12
    cubes_per_step = 150

    for step in range(num_steps):
        platform_y = 70 - step * 5
        platform_x = -25 + step * 4

        # Platform base
        for px in range(8):
            for pz in range(15):
                color = [0.4, 0.45, 0.5]  # Gray platforms
                scene.add_cube_colored(
                    [platform_x + px * 1.05, platform_y, pz * 1.05 - 7.5],
                    0.5, 2.0, color
                )
                cube_count += 1

        # Cubes on top ready to fall
        rows = 3 if step < 8 else 2
        for row in range(rows):
            for col in range(12):
                # Colorful cubes
                hue = (step * 12 + col) / (num_steps * 12)
                color = [
                    0.5 + 0.5 * math.sin(hue * 6 * math.pi),
                    0.5 + 0.5 * math.sin(hue * 6 * math.pi + 2.09),
                    0.5 + 0.5 * math.sin(hue * 6 * math.pi + 4.19),
                ]
                scene.add_cube_colored(
                    [platform_x + 1 + row * 1.5, platform_y + 1.5 + random.uniform(0, 0.5),
                     col * 1.05 - 6],
                    0.45, 0.7, color
                )
                cube_count += 1

    # Trigger balls pushing from top
    for i in range(10):
        z = i * 1.2 - 5
        scene.add_sphere_with_velocity_colored(
            [-35, 75, z], [25, -5, random.uniform(-3, 3)],
            1.8, 30, [0.9, 0.4, 0.2]
        )

    # Side impact balls
    for i in range(8):
        y = 40 + i * 4
        scene.add_sphere_with_velocity_colored(
            [30, y, 0], [-40, 0, random.uniform(-5, 5)],
            2.2, 45, [0.3, 0.7, 0.9]
        )

    cubes, spheres = scene.shape_counts()
    print(f"  {cubes} cubes, {spheres} balls")

    sim = physobx.Simulator(scene, width=2160, height=3840)
    sim.set_camera([40.0, 40.0, 50.0], [-5.0, 30.0, 0.0])

    return sim, output_dir + "/scene3_waterfall"


def scene_4_pyramid_explosion(output_dir):
    """Giant pyramid exploding from within."""
    print("\n=== Scene 4: Pyramid Explosion ===")

    scene = physobx.Scene()
    scene.add_ground(0.0, 150.0)

    cube_count = 0
    pyramid_base = 25

    # Build pyramid layer by layer
    for y in range(pyramid_base):
        layer_size = pyramid_base - y
        offset = y / 2

        for x in range(layer_size):
            for z in range(layer_size):
                # Sandy/golden gradient
                t = y / pyramid_base
                color = [
                    0.85 - t * 0.2,
                    0.65 - t * 0.15,
                    0.25 + t * 0.1,
                ]
                # Add some variation
                color = [c + random.uniform(-0.05, 0.05) for c in color]

                scene.add_cube_colored(
                    [(x + offset) * 1.05 - pyramid_base * 0.525 + 0.525,
                     y * 1.05 + 0.525,
                     (z + offset) * 1.05 - pyramid_base * 0.525 + 0.525],
                    0.5, 1.2, color
                )
                cube_count += 1

    # Internal explosion - balls bursting outward from center
    for i in range(25):
        angle_h = random.uniform(0, 2 * math.pi)
        angle_v = random.uniform(-0.3, 0.8)

        speed = 40 + random.uniform(0, 20)
        vx = math.cos(angle_h) * math.cos(angle_v) * speed
        vy = math.sin(angle_v) * speed + 20
        vz = math.sin(angle_h) * math.cos(angle_v) * speed

        color = [0.9, 0.3 + random.uniform(0, 0.3), 0.1]
        scene.add_sphere_with_velocity_colored(
            [0, 8, 0], [vx, vy, vz], 2.0 + random.uniform(0, 1), 50, color
        )

    # Aerial bombardment
    for i in range(15):
        x = (i % 5 - 2) * 8
        z = (i // 5 - 1) * 8
        scene.add_sphere_with_velocity_colored(
            [x, 80, z], [random.uniform(-5, 5), -45, random.uniform(-5, 5)],
            2.5, 55, [0.7, 0.5, 0.9]
        )

    cubes, spheres = scene.shape_counts()
    print(f"  {cubes} cubes, {spheres} balls")

    sim = physobx.Simulator(scene, width=2160, height=3840)
    sim.set_camera([50.0, 35.0, 50.0], [0.0, 12.0, 0.0])

    return sim, output_dir + "/scene4_pyramid"


def scene_5_twin_towers(output_dir):
    """Twin towers with synchronized destruction."""
    print("\n=== Scene 5: Twin Towers ===")

    scene = physobx.Scene()
    scene.add_ground(0.0, 150.0)

    cube_count = 0
    tower_height = 60
    tower_width = 5
    tower_spacing = 20

    # Build two towers
    for tower in [-1, 1]:
        tower_x = tower * tower_spacing / 2

        for y in range(tower_height):
            # Slight taper
            width = max(3, tower_width - y // 25)
            offset = (tower_width - width) / 2

            for x in range(width):
                for z in range(width):
                    # Blue-glass gradient
                    t = y / tower_height
                    if tower == -1:
                        color = [0.3 + t * 0.2, 0.5 + t * 0.3, 0.8 - t * 0.1]
                    else:
                        color = [0.8 - t * 0.1, 0.5 + t * 0.3, 0.3 + t * 0.2]

                    scene.add_cube_colored(
                        [tower_x + (x + offset) * 1.05 - tower_width * 0.525 + 0.525,
                         y * 1.05 + 0.525,
                         (z + offset) * 1.05 - tower_width * 0.525 + 0.525],
                        0.5, 0.9, color
                    )
                    cube_count += 1

    # Bridge between towers at mid-height
    bridge_y = 30
    for bx in range(-8, 9):
        for bz in range(3):
            color = [0.6, 0.6, 0.65]
            scene.add_cube_colored(
                [bx * 1.05, bridge_y + bz * 1.05, 0],
                0.5, 1.0, color
            )
            cube_count += 1

    # Synchronized attack - balls from both sides
    for i in range(12):
        height = 15 + i * 4
        speed = 50

        # Left tower attack
        scene.add_sphere_with_velocity_colored(
            [-50, height, 0], [speed, 0, random.uniform(-3, 3)],
            2.0, 40, [0.9, 0.4, 0.2]
        )
        # Right tower attack
        scene.add_sphere_with_velocity_colored(
            [50, height, 0], [-speed, 0, random.uniform(-3, 3)],
            2.0, 40, [0.2, 0.4, 0.9]
        )

    # Top bombers
    for i in range(10):
        x = (i - 5) * 5
        scene.add_sphere_with_velocity_colored(
            [x, 90, 0], [0, -50, 0], 2.5, 55, [0.8, 0.3, 0.8]
        )

    cubes, spheres = scene.shape_counts()
    print(f"  {cubes} cubes, {spheres} balls")

    sim = physobx.Simulator(scene, width=2160, height=3840)
    sim.set_camera([55.0, 40.0, 55.0], [0.0, 30.0, 0.0])

    return sim, output_dir + "/scene5_twins"


def render_scene(sim, scene_path, fps=60, duration=10.0):
    """Render a scene to frames."""
    os.makedirs(scene_path, exist_ok=True)

    total_frames = int(fps * duration)
    print(f"  Rendering {total_frames} frames ({duration}s at {fps}fps)...")

    for frame in range(total_frames):
        sim.step(1.0 / fps)
        filename = f"{scene_path}/frame_{frame:04d}.png"
        sim.save_png(filename)
        if frame % 60 == 0:
            print(f"    Frame {frame}/{total_frames}")

    return scene_path


def create_video(scene_path, fps=60):
    """Convert frames to MP4 video."""
    video_path = f"{scene_path}/video.mp4"
    cmd = [
        "ffmpeg", "-y", "-framerate", str(fps),
        "-i", f"{scene_path}/frame_%04d.png",
        "-c:v", "libx264", "-pix_fmt", "yuv420p", "-crf", "18",
        video_path
    ]
    print(f"  Creating video: {video_path}")
    subprocess.run(cmd, capture_output=True)
    return video_path


def main():
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    output_dir = f"./render/instagram_4k_{timestamp}"
    os.makedirs(output_dir, exist_ok=True)
    print(f"Output folder: {output_dir}")
    print("Resolution: 2160x3840 (4K Portrait 9:16)")

    random.seed(42)

    scenes = [
        scene_1_tower_collapse,
        scene_2_domino_cascade,
        scene_3_cube_waterfall,
        scene_4_pyramid_explosion,
        scene_5_twin_towers,
    ]

    videos = []

    for i, scene_func in enumerate(scenes, 1):
        print(f"\n{'='*50}")
        print(f"SCENE {i}/5")
        print(f"{'='*50}")

        sim, scene_path = scene_func(output_dir)
        render_scene(sim, scene_path)
        video_path = create_video(scene_path)
        videos.append(video_path)

    print(f"\n{'='*50}")
    print("ALL DONE!")
    print(f"{'='*50}")
    print(f"\nVideos created:")
    for v in videos:
        size_mb = os.path.getsize(v) / (1024 * 1024)
        print(f"  {v} ({size_mb:.1f} MB)")


if __name__ == "__main__":
    main()
