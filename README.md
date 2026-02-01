# FOSDEM Video Downloader

A Python tool to download FOSDEM conference videos from an ICS schedule file. It
is designed to work with the ICS export of bookmarked talks from the
[FOSDEM mobile app](https://github.com/cbeyls/fosdem-companion-android).

## Features

- Downloads FOSDEM videos from your bookmarked talks
- Uses the official FOSDEM schedule and video URLs
- Supports concurrent downloads to speed up the process
- Dry-run mode to print URLs without downloading

## Installation

This project uses [uv](https://github.com/astral-sh/uv) for dependency and
environment management.

```bash
git clone https://github.com/butlerx/fosdem-video-downloader.git
cd fosdem-video-downloader

uv sync
```

This will create a virtual environment and install all dependencies.

## Usage

The CLI is exposed via the package entry point.

Basic usage:

```bash
uv run python fosdem_video.py bookmarks.ics
```

Advanced options (example):

```bash
uv run python fosdem_video.py bookmarks.ics \
  --output-dir ~/videos/fosdem \
  --workers 4
```

### Command Line Arguments

- `ics_file`: Path to the FOSDEM schedule ICS file (required)
- `-o, --output-dir`: Directory to save downloaded videos (default:
  `fosdem_videos`)
- `-w, --workers`: Number of concurrent downloads (default: `3`)
- `--dry-run`: Print video URLs without downloading

## Getting Your Bookmarks

1. Use the FOSDEM mobile app to bookmark talks you're interested in.
2. Export your bookmarks as an ICS file from the app.
3. Use this ICS file as input for the tool.

## Development

Install development dependencies:

```bash
uv sync --group dev
```

Run type checking and linting:

```bash
uv run ty check
uv run ruff check --fix
uv run ruff format
```

## License

Apache License 2.0

## Contributing

Contributions are welcome! Please feel free to submit a pull request.
