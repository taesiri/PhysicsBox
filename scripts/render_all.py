#!/usr/bin/env python3
"""Render all example scenes and convert to videos."""

import subprocess
import sys
from pathlib import Path

EXAMPLES = [
    "falling_cubes.py",
    "cube_tower.py",
    "cube_rain.py",
    "cube_pyramid.py",
]

def frames_to_video(frame_dir: Path, fps: int = 60) -> Path:
    """Convert PNG frames to MP4 video using ffmpeg."""
    output_video = frame_dir.parent / f"{frame_dir.name}.mp4"

    # Detect frame pattern
    if list(frame_dir.glob("frame_0000.png")):
        pattern = "frame_%04d.png"
    elif list(frame_dir.glob("frame_000.png")):
        pattern = "frame_%03d.png"
    else:
        raise FileNotFoundError(f"No frame_*.png files in {frame_dir}")

    cmd = [
        "ffmpeg", "-y",
        "-framerate", str(fps),
        "-i", str(frame_dir / pattern),
        "-c:v", "libx264",
        "-preset", "medium",
        "-crf", "18",
        "-pix_fmt", "yuv420p",
        "-movflags", "+faststart",
        str(output_video)
    ]

    subprocess.run(cmd, check=True, capture_output=True)
    return output_video


def main():
    project_root = Path(__file__).parent.parent
    examples_dir = project_root / "examples"
    render_dir = project_root / "render"

    print("=" * 60)
    print("Rendering all example scenes")
    print("=" * 60)

    rendered_folders = []

    for example in EXAMPLES:
        example_path = examples_dir / example
        if not example_path.exists():
            print(f"Skipping {example} (not found)")
            continue

        print(f"\n>>> Running {example}...")

        # Get list of folders before running
        before = set(render_dir.glob("*")) if render_dir.exists() else set()

        # Run the example
        result = subprocess.run(
            [sys.executable, str(example_path)],
            cwd=project_root,
            capture_output=True,
            text=True
        )

        if result.returncode != 0:
            print(f"Error running {example}:")
            print(result.stderr)
            continue

        print(result.stdout)

        # Find newly created folder
        after = set(render_dir.glob("*")) if render_dir.exists() else set()
        new_folders = after - before
        rendered_folders.extend(new_folders)

    print("\n" + "=" * 60)
    print("Converting frames to videos")
    print("=" * 60)

    videos = []
    for folder in rendered_folders:
        if folder.is_dir():
            print(f"\n>>> Converting {folder.name}...")
            try:
                video_path = frames_to_video(folder)
                videos.append(video_path)
                print(f"    Created: {video_path}")
            except Exception as e:
                print(f"    Error: {e}")

    print("\n" + "=" * 60)
    print("Summary")
    print("=" * 60)
    print(f"Rendered {len(rendered_folders)} scenes")
    print(f"Created {len(videos)} videos:")
    for v in videos:
        print(f"  - {v}")


if __name__ == "__main__":
    main()
