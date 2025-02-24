import argparse
import contextlib
import logging
from concurrent.futures import ThreadPoolExecutor
from os import path, remove
from pathlib import Path
from sys import stdout
from typing import List, NamedTuple
from urllib.parse import urlparse

import requests
from icalendar import Calendar

logger = logging.getLogger(__name__)


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
        logger.info("Starting download: %s", output_path.name)
        response = requests.get(url, stream=True)
        response.raise_for_status()

        total_size = int(response.headers.get("content-length", 0))
        logging.debug("%s is %d MB", output_path.name, total_size)
        block_size = 1024 * 1024  # 1MB chunks

        with open(output_path, "wb") as f:
            for chunk in response.iter_content(block_size):
                f.write(chunk)

        logger.info("Downloaded %s", output_path.name)
        return True
    except Exception as e:
        logger.error("Failed to download %s: %s", url, e)
        with contextlib.suppress(FileNotFoundError):
            # If something happened mid download we should remove the incomplete file
            remove(output_path)
        return False


def is_downloaded(output_dir: str, talk: Talk) -> bool:
    file_path = Path(f"{output_dir}/{talk.year}/{talk.id}.mp4")
    if path.exists(file_path):
        logger.debug("skipping %s as the file already exists", talk.id)
        return True
    return False


def create_dirs(output_dir: str, talks: List[Talk]) -> None:
    output_path = Path(output_dir)
    output_path.mkdir(exist_ok=True)
    years = set([talk.year for talk in talks])
    for year in years:
        video_folder = Path(f"{output_dir}/{year}")
        video_folder.mkdir(exist_ok=True)


def download_fosdem_videos(
    talks: List[Talk],
    output_dir: str = "fosdem_videos",
    num_workers: int = 3,
) -> List[bool]:
    def process_video(talk: Talk) -> bool:
        file_path = Path(f"{output_dir}/{talk.year}/{talk.id}.mp4")
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

    parser.add_argument(
        "--log-level",
        default="INFO",
        choices=["CRITICAL", "ERROR", "WARNING", "INFO", "DEBUG"],
        help="Set the logging output level",
    )

    args = parser.parse_args()

    # Validate the ICS file exists
    if not args.ics_file.exists():
        parser.error(f"ICS file not found: {args.ics_file}")

    return args


def main():
    args = parse_arguments()
    logging.basicConfig(
        level=getattr(logging, args.log_level),
        format="%(asctime)s - %(levelname)s: %(message)s",
        datefmt="%Y-%m-%dT%H:%M:%S",
    )
    logger.info("Parsing ICS file %s", args.ics_file)
    talks = parse_ics_file(args.ics_file)
    logging.info("Found %s talks in Calendar", len(talks))
    talks = [talk for talk in talks if not is_downloaded(args.output_dir, talk)]
    logging.info("Found %s videos to download", len(talks))
    if args.dry_run:
        urls = "\n".join([f"  - {talk.url}" for talk in talks])
        stdout.write(f"List of talks videos: \n{urls}\n")
        return
    results = download_fosdem_videos(
        talks,
        output_dir=args.output_dir,
        num_workers=args.workers,
    )
    succesful = len([r for r in results if r])
    logging.info("Downloaded %s of %s talks", succesful, len(talks))


if __name__ == "__main__":
    main()
