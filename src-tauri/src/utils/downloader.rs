use anyhow::anyhow;
use log::info;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::types::error::CustomError;

#[derive(Debug, Deserialize, Serialize)]
struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

pub fn build_client() -> Result<Client, CustomError> {
    let client = Client::builder().user_agent("sbrw-utility").build()?;
    Ok(client)
}

pub fn download_using_url(
    client: &Client,
    url: &str,
    dest: &Path,
    file_name: &str,
) -> Result<PathBuf, CustomError> {
    //Downloads file using url and stores in local destination path

    let response = client.get(url).send()?;

    let tmp_path = dest.join(file_name);
    let mut out = std::fs::File::create(&tmp_path)?;
    let mut content = std::io::Cursor::new(response.bytes()?);
    std::io::copy(&mut content, &mut out)?;

    Ok(tmp_path)
}

pub fn get_latest_download_url(
    client: &Client,
    repo_url: &str,
    starting_name: &str,
) -> Result<String, CustomError> {
    let release: Release = client
        .get(repo_url)
        .header("User-Agent", "sbrw-utility")
        .send()?
        .json()?;

    if let Some(asset) = release
        .assets
        .iter()
        .find(|a| a.name.starts_with(starting_name))
    {
        info!("{:?}", asset.browser_download_url);
        Ok(asset.browser_download_url.clone())
    } else {
        return Err(CustomError::Anyhow(anyhow!(
            "Latest release file not found"
        )));
    }
}
