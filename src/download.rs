use crate::{error::DownloadError, talk::Talk};
use log::{debug, error, info, warn};
use reqwest::{StatusCode, blocking::Client};
use std::{
    fs::{self, File},
    io,
    path::{Path, PathBuf},
};

pub fn output_path(output_dir: &Path, talk: &Talk) -> PathBuf {
    output_dir.join(&talk.year).join(format!("{}.mp4", talk.id))
}

pub fn download_video(client: &Client, talk: &Talk, output_dir: &Path) -> bool {
    let path = output_path(output_dir, talk);
    info!("Starting download: {}", path.display());

    request_video(client, talk)
        .and_then(|response| {
            response.map_or(Ok(false), |response| {
                save_video(response, &path).map(|()| true)
            })
        })
        .unwrap_or_else(|err| {
            error!("Failed to download {}: {err}", talk.url);
            cleanup_incomplete_file(&path);
            false
        })
}

fn request_video(
    client: &Client,
    talk: &Talk,
) -> Result<Option<reqwest::blocking::Response>, DownloadError> {
    client
        .get(&talk.url)
        .send()
        .map_err(|source| DownloadError::Request(talk.url.clone(), source))
        .and_then(|response| match response.status() {
            StatusCode::NOT_FOUND => {
                warn!("Video not found (404): {}", talk.url);
                Ok(None)
            }
            _ => response
                .error_for_status()
                .map(Some)
                .map_err(|source| DownloadError::Response(talk.url.clone(), source)),
        })
}

fn save_video(mut response: reqwest::blocking::Response, path: &Path) -> Result<(), DownloadError> {
    debug!(
        "{} is {} MB",
        path.display(),
        response.content_length().unwrap_or(0)
    );
    let mut file =
        File::create(path).map_err(|source| DownloadError::CreateFile(path.to_owned(), source))?;
    io::copy(&mut response, &mut file)
        .map_err(|source| DownloadError::WriteFile(path.to_owned(), source))?;
    info!("Downloaded {}", path.display());
    Ok(())
}

fn cleanup_incomplete_file(path: &Path) {
    fs::remove_file(path)
        .err()
        .filter(|error| error.kind() != io::ErrorKind::NotFound)
        .map(|source| DownloadError::RemoveIncompleteFile(path.to_owned(), source))
        .inspect(|error| warn!("{error}"));
}
