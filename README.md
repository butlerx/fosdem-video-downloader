# FOSDEM Video Downloader

A Python script to download FOSDEM conference videos from an ICS schedule file.
Designed to work with the ICS export from the
[FOSDEM mobile app's](https://github.com/cbeyls/fosdem-companion-android)
bookmarked talks.

## Features

- Downloads FOSDEM videos from your bookmarked talks
- Supports concurrent downloads to speed up the process

## Installation

1. Clone the repository:

```bash
git clone https://github.com/butlerx/fosdem-video-downloader.git
cd fosdem-video-downloader
```

2. Create a virtual environment (optional but recommended):

```bash
python -m venv venv
source venv/bin/activate
```

3. Install dependencies:

```bash
pip install -r requirements.txt
```

## Usage

Basic usage:

```bash
python fosdem_video.py bookmarks.ics
```

Advanced options:

```bash
python fosdem_video.py bookmarks.ics -o ~/videos/fosdem -w 4
```

### Command Line Arguments

- `ics_file`: Path to the FOSDEM schedule ICS file (required)
- `-o, --output-dir`: Directory to save downloaded videos (default:
  "fosdem_videos")
- `-w, --workers`: Number of concurrent downloads (default: 3)
- `--dry-run`: Print video URLs without downloading

## Getting Your Bookmarks

1. Use the FOSDEM mobile app to bookmark talks you're interested in
2. Export your bookmarks as an ICS file from the app
3. Use this ICS file as input for the script

## License

Apache License 2.0

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
