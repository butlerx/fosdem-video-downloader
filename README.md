# FOSDEM Video Downloader

A Rust CLI to download FOSDEM conference videos from an ICS schedule file. It
is designed to work with the ICS export of bookmarked talks from the
[FOSDEM mobile app](https://github.com/cbeyls/fosdem-companion-android).

## Features

- Downloads FOSDEM videos from your bookmarked talks
- Uses the official FOSDEM schedule and video URLs
- Supports concurrent downloads to speed up the process
- Dry-run mode to print URLs without downloading

## Installation

This project uses [Cargo](https://doc.rust-lang.org/cargo/) for dependency and
build management. Install Rust with [rustup](https://rustup.rs/) if necessary.

```bash
git clone https://github.com/butlerx/fosdem-video-downloader.git
cd fosdem-video-downloader

cargo build --release
```

The compiled binary will be at `target/release/fosdem-video-downloader`.

## Usage

The CLI is exposed via the package entry point.

Basic usage:

```bash
cargo run -- bookmarks.ics
```

Advanced options (example):

```bash
cargo run -- bookmarks.ics \
  --output-dir ~/videos/fosdem \
  --workers 4
```

### Command Line Arguments

- `ics_file`: Path to the FOSDEM schedule ICS file (required)
- `-o, --output-dir`: Directory to save downloaded videos (default:
  `fosdem_videos`)
- `-w, --workers`: Number of concurrent downloads (default: `3`)
- `--dry-run`: Print video URLs without downloading
- `--log-level`: Set logging verbosity: `CRITICAL`, `ERROR`, `WARNING`,
  `INFO`, or `DEBUG` (default: `INFO`)

## Getting Your Bookmarks

1. Use the FOSDEM mobile app to bookmark talks you're interested in.
2. Export your bookmarks as an ICS file from the app.
3. Use this ICS file as input for the tool.

## Development

Run the tests and formatting checks:

```bash
cargo test
cargo fmt --check
```

## Releases

Releases are managed automatically via
[release-please](https://github.com/googleapis/release-please). When commits
following [Conventional Commits](https://www.conventionalcommits.org/) are
pushed to `main`, release-please opens a PR that bumps the version and updates
`CHANGELOG.md`. Merging that PR triggers a GitHub Release and publishes to
crates.io using [trusted publishing](https://crates.io/docs/trusted-publishing).

Before the first automated release, publish the crate once manually, then add a
trusted publisher in its crates.io settings for this repository and
`.github/workflows/release.yml`.

## License

Apache License 2.0

## Contributing

Contributions are welcome! Please feel free to submit a pull request.
