//! Download FOSDEM videos listed in an ICS schedule export.
#![warn(clippy::pedantic)]

mod cli;
mod download;
mod error;
mod ics;
mod talk;

use crate::{cli::Args, download::output_path, error::AppError};
use clap::Parser;
use log::{debug, info};
use rayon::{ThreadPoolBuilder, prelude::*};
use reqwest::blocking::Client;
use std::{fs, process::ExitCode, time::Duration};

fn main() -> ExitCode {
    run().map_or_else(
        |error| {
            eprintln!("Error: {error}");
            ExitCode::FAILURE
        },
        |()| ExitCode::SUCCESS,
    )
}

fn run() -> Result<(), AppError> {
    let args = Args::parse();
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or(args.log_level.to_lowercase()),
    )
    .format_timestamp_secs()
    .init();

    info!("Parsing ICS file {}", args.ics_file.display());
    let talks = ics::parse_file(&args.ics_file)?;
    info!("Found {} talks in Calendar", talks.len());
    let talks: Vec<_> = talks
        .into_iter()
        .filter(|talk| {
            let path = output_path(&args.output_dir, talk);
            let exists = path.exists();
            if exists {
                debug!("Skipping {} as the file already exists", talk.id);
            }
            !exists
        })
        .collect();
    info!("Found {} videos to download", talks.len());

    if args.dry_run {
        let formatted_talks = talks
            .iter()
            .map(|talk| format!("  - {}", talk.url))
            .collect::<Vec<_>>()
            .join("\n");
        println!("List of talks videos:\n{formatted_talks}");
        return Ok(());
    }

    for talks in &talks {
        let path = args.output_dir.join(&talks.year);
        fs::create_dir_all(&path)
            .map_err(|source| AppError::CreateOutputDirectory(path, source))?;
    }

    let client = Client::builder().timeout(Duration::from_secs(10)).build()?;
    let results = ThreadPoolBuilder::new()
        .num_threads(args.workers)
        .build()?
        .install(|| {
            talks
                .par_iter()
                .map(|talk| download::download_video(&client, talk, &args.output_dir))
                .collect::<Vec<_>>()
        });
    info!(
        "Downloaded {} of {} talks",
        results.iter().filter(|&&success| success).count(),
        results.len()
    );
    Ok(())
}
