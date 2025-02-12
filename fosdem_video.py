import contextlib
from icalendar import Calendar
import argparse
import requests
from os import path, remove
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
from urllib.parse import urlparse
from typing import List, NamedTuple
from urllib.parse import urlparse


class Talk(NamedTuple):
    url: str
    year: str
    id: str


def get_path_elements(url: str) -> tuple[str, str]:
    parsed = urlparse(url)
    path_parts = [p for p in parsed.path.split("/") if p]

    if not path_parts:
        return ("", "")

    return (path_parts[0], path_parts[-1])


def parse_ics_file(ics_path: str) -> List[Talk]:
    """Extract video information from ICS file"""
    with open(ics_path, "rb") as f:
        cal = Calendar.from_ical(f.read())

    talks = []
    for event in cal.walk("vevent"):
        url = event.get("url")
        location = event.get("location")
        if not url or not location:
            continue

        # Convert to string if it's a vText object
        url = str(url)
        year, talk_id = get_path_elements(str(url))
        location = str(location).replace(".", "").split(" ", 1)[0].lower()

        video_url = f"https://video.fosdem.org/{year}/{location}/{talk_id}.mp4"

        talks.append(Talk(video_url, year, talk_id))

    return talks


def download_video(url: str, output_path: Path) -> bool:
    try:
        print(f"Starting download: {output_path.name}")
        response = requests.get(url, stream=True)
        response.raise_for_status()

        total_size = int(response.headers.get("content-length", 0))
        print(f"{output_path.name} is {total_size} MB")
        block_size = 1024 * 1024  # 1MB chunks

        with open(output_path, "wb") as f:
            for chunk in response.iter_content(block_size):
                f.write(chunk)

        print(f"Downloaded {output_path.name}")
        return True
    except Exception as e:
        print(f"Failed to download {url}: {str(e)}")
        with contextlib.suppress(FileNotFoundError):
            # If something happened mid download we should remove the incomplete file
            remove(output_path)
        return False


def download_fosdem_videos(
    talks: List[Talk],
    output_dir: str = "fosdem_videos",
    num_workers: int = 3,
):
    output_path = Path(output_dir)
    output_path.mkdir(exist_ok=True)

    def process_video(talk: Talk) -> bool:
        video_folder = Path(f"{output_dir}/{talk.year}")
        video_folder.mkdir(exist_ok=True)
        file_path = Path(f"{video_folder}/{talk.id}.mp4")
        if path.exists(file_path):
            print(f"skipping {talk.id} as the file already exists")
            return True

        return download_video(talk.url, file_path)

    with ThreadPoolExecutor(max_workers=num_workers) as executor:
        results = list(executor.map(process_video, talks))

    return results


def parse_arguments():
    parser = argparse.ArgumentParser(
        description="Download FOSDEM videos from ICS schedule",
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
    )

    parser.add_argument(
        "ics_file", type=Path, help="Path to the FOSDEM schedule ICS file"
    )

    parser.add_argument(
        "-o",
        "--output-dir",
        type=Path,
        default=Path("fosdem_videos"),
        help="Directory to save downloaded videos",
    )

    parser.add_argument(
        "-w", "--workers", type=int, default=3, help="Number of concurrent downloads"
    )

    parser.add_argument(
        "--dry-run", action="store_true", help="Print video URLs without downloading"
    )

    args = parser.parse_args()

    # Validate the ICS file exists
    if not args.ics_file.exists():
        parser.error(f"ICS file not found: {args.ics_file}")

    return args


def main():
    args = parse_arguments()
    print(f"Parsing ICS file {args.ics_file}")
    talks = parse_ics_file(args.ics_file)
    print(f"Found {len(talks)} videos to download")
    if args.dry_run:
        print("List of talks videos:")
        for talk in talks:
            print(talk.url)
        return
    results = download_fosdem_videos(
        talks,
        output_dir=args.output_dir,
        num_workers=args.workers,
    )
    succesful = len([r for r in results if r])
    print(f"downloaded {succesful} of {len(talks)} talks")


if __name__ == "__main__":
    main()
