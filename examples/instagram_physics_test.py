"""Instagram 4K Portrait - Physics Improved Test (with CCD + substeps)"""

import physobx
import os
import subprocess
import math
import random
from datetime import datetime


def scene_1_wrecking_ball_tower(output_dir):
    """Single massive wrecking ball vs tall tower - tests CCD."""
    print("\n=== Scene 1: Wrecking Ball vs Tower ===")

    scene = physobx.Scene()
    scene.add_ground(0.0, 100.0)

    # Build a dense tower (8x8 base, 50 tall)
    tower_height = 50
    tower_width = 8

    for y in range(tower_height):
        width = max(4, tower_width - y // 15)
        offset = (tower_width - width) / 2

        for x in range(width):
            for z in range(width):
                # Gradient from warm to cool
                t = y / tower_height
                color = [
                    0.9 - t * 0.5,
                    0.4 + t * 0.3,
                    0.2 + t * 0.6,
                ]
                scene.add_cube_colored(
                    [(x + offset) * 1.05 - tower_width * 0.525 + 0.525,
                     y * 1.05 + 0.525,
                     (z + offset) * 1.05 - tower_width * 0.525 + 0.525],
                    0.5, 1.0, color
                )

    # One massive wrecking ball - high speed to test CCD
    scene.add_sphere_with_velocity_colored(
        [45, 25, 0],      # Start far away
        [-60, 0, 0],      # High speed toward tower
        5.0,              # Large radius
        500.0,            # Very heavy
        [0.3, 0.3, 0.35]  # Dark metallic
    )

    # Secondary balls from above
    for i in range(6):
        x = (i % 3 - 1) * 4
        z = (i // 3 - 0.5) * 4
        scene.add_sphere_with_velocity_colored(
            [x, 80, z],
            [random.uniform(-3, 3), -40, random.uniform(-3, 3)],
            2.0, 60, [0.8, 0.4, 0.2]
        )

    cubes, spheres = scene.shape_counts()
    print(f"  {cubes} cubes, {spheres} balls")

    sim = physobx.Simulator(scene, width=2160, height=3840)
    sim.set_camera([40, 35, 40], [0, 25, 0])

    return sim, output_dir + "/scene1_wrecking"


def scene_2_ball_rain(output_dir):
    """Many balls raining down on cube structures - tests solver iterations."""
    print("\n=== Scene 2: Ball Rain ===")

    scene = physobx.Scene()
    scene.add_ground(0.0, 100.0)

    # Build multiple small platforms at different heights
    platforms = [
        (0, 0, 10, 6),     # x, z, y, size
        (-12, 8, 20, 5),
        (12, -8, 30, 5),
        (-8, -12, 40, 4),
        (8, 12, 50, 4),
    ]

    for px, pz, py, size in platforms:
        for x in range(size):
            for z in range(size):
                for layer in range(3):
                    # Rainbow based on height
                    hue = py / 60
                    color = [
                        0.5 + 0.5 * math.sin(hue * 2 * math.pi),
                        0.5 + 0.5 * math.sin(hue * 2 * math.pi + 2.09),
                        0.5 + 0.5 * math.sin(hue * 2 * math.pi + 4.19),
                    ]
                    scene.add_cube_colored(
                        [px + x * 1.05 - size * 0.525,
                         py + layer * 1.05,
                         pz + z * 1.05 - size * 0.525],
                        0.5, 0.8, color
                    )

    # Rain of balls from above
    for i in range(50):
        x = random.uniform(-20, 20)
        z = random.uniform(-20, 20)
        y = 70 + random.uniform(0, 30)

        # Colorful balls
        hue = i / 50
        color = [
            0.6 + 0.4 * math.sin(hue * 4 * math.pi),
            0.6 + 0.4 * math.sin(hue * 4 * math.pi + 2.09),
            0.6 + 0.4 * math.sin(hue * 4 * math.pi + 4.19),
        ]

        scene.add_sphere_with_velocity_colored(
            [x, y, z],
            [random.uniform(-5, 5), -30 - random.uniform(0, 20), random.uniform(-5, 5)],
            1.0 + random.uniform(0, 1.0),
            15 + random.uniform(0, 20),
            color
        )

    cubes, spheres = scene.shape_counts()
    print(f"  {cubes} cubes, {spheres} balls")

    sim = physobx.Simulator(scene, width=2160, height=3840)
    sim.set_camera([45, 50, 45], [0, 30, 0])

    return sim, output_dir + "/scene2_rain"


def scene_3_domino_chain(output_dir):
    """Long domino chain with ball trigger - tests collision accuracy."""
    print("\n=== Scene 3: Domino Chain Reaction ===")

    scene = physobx.Scene()
    scene.add_ground(0.0, 150.0)

    # Spiral domino chain going upward
    num_dominoes = 300
    spiral_turns = 4
    max_height = 55

    for i in range(num_dominoes):
        t = i / num_dominoes
        angle = t * spiral_turns * 2 * math.pi
        radius = 6 + t * 22
        height = t * max_height

        x = math.cos(angle) * radius
        z = math.sin(angle) * radius

        # Rainbow gradient
        hue = t
        color = [
            0.5 + 0.5 * math.sin(hue * 3 * math.pi),
            0.5 + 0.5 * math.sin(hue * 3 * math.pi + 2.09),
            0.5 + 0.5 * math.sin(hue * 3 * math.pi + 4.19),
        ]

        # Tall thin domino (3 cubes stacked)
        for dy in range(3):
            scene.add_cube_colored(
                [x, height + dy * 1.05 + 0.525, z],
                0.5, 0.6, color
            )

    # Trigger ball at the start - high precision needed
    start_angle = 0.05  # Slightly offset
    start_radius = 4
    scene.add_sphere_with_velocity_colored(
        [math.cos(start_angle) * start_radius, 3, math.sin(start_angle) * start_radius],
        [12, 0, 3],  # Push toward first domino
        2.5, 100, [1.0, 0.2, 0.1]
    )

    # Additional chaos balls from above
    for i in range(8):
        angle = i * 2 * math.pi / 8
        r = 15 + (i % 3) * 5
        h = 70 + (i % 4) * 8
        scene.add_sphere_with_velocity_colored(
            [math.cos(angle) * r, h, math.sin(angle) * r],
            [-math.cos(angle) * 25, -20, -math.sin(angle) * 25],
            1.8, 30, [0.3, 0.6 + i * 0.04, 0.9]
        )

    cubes, spheres = scene.shape_counts()
    print(f"  {cubes} cubes, {spheres} balls")

    sim = physobx.Simulator(scene, width=2160, height=3840)
    sim.set_camera([50, 45, 50], [0, 28, 0])

    return sim, output_dir + "/scene3_domino"


def render_scene(sim, scene_path, fps=60, duration=10.0, substeps=2):
    """Render a scene with physics substeps."""
    os.makedirs(scene_path, exist_ok=True)

    total_frames = int(fps * duration)
    print(f"  Rendering {total_frames} frames ({duration}s at {fps}fps, {substeps} substeps)...")

    for frame in range(total_frames):
        # Use substeps for better physics accuracy
        sim.step(1.0 / fps, substeps)

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
    output_dir = f"./render/instagram_physics_{timestamp}"
    os.makedirs(output_dir, exist_ok=True)
    print(f"Output folder: {output_dir}")
    print("Resolution: 2160x3840 (4K Portrait 9:16)")
    print("Physics: CCD enabled, 8 solver iterations, 2 substeps")

    random.seed(42)

    scenes = [
        scene_1_wrecking_ball_tower,
        scene_2_ball_rain,
        scene_3_domino_chain,
    ]

    videos = []

    for i, scene_func in enumerate(scenes, 1):
        print(f"\n{'='*50}")
        print(f"SCENE {i}/3")
        print(f"{'='*50}")

        sim, scene_path = scene_func(output_dir)
        render_scene(sim, scene_path, substeps=2)
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
