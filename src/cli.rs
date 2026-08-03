use crate::error::WorkerCountError;
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(about = "Download FOSDEM videos from an ICS schedule")]
pub struct Args {
    /// Path to the FOSDEM schedule ICS file
    pub ics_file: PathBuf,

    /// Directory to save downloaded videos
    #[arg(short, long, default_value = "fosdem_videos")]
    pub output_dir: PathBuf,

    /// Number of concurrent downloads
    #[arg(short, long, default_value_t = 3, value_parser = positive_workers)]
    pub workers: usize,

    /// Print video URLs without downloading
    #[arg(long)]
    pub dry_run: bool,

    /// Set the logging output level (CRITICAL, ERROR, WARNING, INFO, or DEBUG)
    #[arg(long, default_value = "INFO")]
    pub log_level: String,
}

fn positive_workers(value: &str) -> Result<usize, String> {
    value
        .parse::<usize>()
        .map_err(|_| WorkerCountError::InvalidInteger.to_string())
        .and_then(|workers| {
            (workers > 0)
                .then_some(workers)
                .ok_or_else(|| WorkerCountError::Zero.to_string())
        })
}
