//! Filer.net downloader. Responsible to download files from Filer and
//! checking the status of a download link.
//!
//! Right now only Premium Accounts are supported.

use crate::bus::{Message, MessageBus};
use crate::config::Config;
use crate::error::*;
use crate::loader::Loader;
use crate::models::{DownloadFile, FileHash, FileHoster, FileStatus, SmartDownloadList};
use md5::{Digest, Md5};
use reqwest;
use std::fs::File;

/// Filer downloader struct which allows to download files from filer with an premium account.
#[derive(Clone)]
pub struct Filer {
    config: Config,
    d_list: SmartDownloadList,
    bus: MessageBus,
}

impl Loader for Filer {
    /// This function updates an filer account with the actual status
    fn update_account(&self, account: &mut crate::config::ConfigAccount) -> Result<()> {
        // This implementation can only check filer accounts
        if account.hoster != crate::config::ConfigHoster::Filer {
            bail!("Not a Filer Account");
        }

        // Get the user status from Filer
        let client = reqwest::Client::new();
        let mut resp = client
            .get("http://api.filer.net/profile.json")
            .basic_auth(&account.username, Some(&account.password))
            .send()?;

        // only continue if the answer was successfull
        if resp.status() != reqwest::StatusCode::OK {
            account.status = crate::config::ConfigAccountStatus::NotValid;
            return Ok(());
        }

        // get the body
        let body = resp.text()?;
        let json: serde_json::Value = serde_json::from_str(&body)?;

        // set the account status
        match json["data"]["state"].as_str() {
            Some("premium") => account.status = crate::config::ConfigAccountStatus::Premium,
            Some("free") => account.status = crate::config::ConfigAccountStatus::Free,
            _ => account.status = crate::config::ConfigAccountStatus::Unknown,
        }

        Ok(())
    }

    /// Check the download url and return the file info
    fn check_url(&self, url: &str) -> Result<Option<DownloadFile>> {
        let r_url = reqwest::Url::parse(url)?;

        if r_url.host_str() != Some("filer.net") {
            bail!("The given link wasn't a share-online download link");
        }

        let req = format!(
            "http://api.filer.net/status/{}.json",
            url.split("get/")
                .collect::<Vec<&str>>()
                .get(1)
                .unwrap_or(&"")
        );
        let mut resp = reqwest::get(&req)?;

        // only continue if the answer was successfull
        if resp.status() != reqwest::StatusCode::OK {
            let mut file = DownloadFile::default();
            file.hoster = FileHoster::Filer;
            file.status = FileStatus::Offline;
            return Ok(Some(file));
        }

        // get the body and split the data
        let body = resp.text()?;
        let json: serde_json::Value = serde_json::from_str(&body)?;

        // create a new file
        let mut file = DownloadFile::default();
        file.hoster = FileHoster::Filer;
        file.url = format!(
            "http://api.filer.net/dl/{}.json",
            json["data"]["hash"].as_str().ok_or("No Hash provided")?
        );
        file.status = FileStatus::Online;
        file.name = json["data"]["name"]
            .as_str()
            .ok_or("No name provided")?
            .to_string();
        file.size = (json["data"]["size"].as_i64().ok_or("No size provided")?) as usize;
        file.hash = FileHash::Md5(
            json["data"]["hash"]
                .as_str()
                .ok_or("No hash provided")?
                .to_string(),
        );

        Ok(Some(file))
    }

    /// Download a file, with this laoder
    fn download(&self, file: &DownloadFile) -> Result<::reqwest::Response> {
         if file.hoster != FileHoster::Filer {
            bail!("Wrong hoster");
        }

        let acc = match self.config.get().get_account(
            crate::config::ConfigHoster::Filer,
            crate::config::ConfigAccountStatus::Premium,
        ) {
            Ok(a) => a,
            Err(_) => {
                return self.free_download(file);
            }
        };

        let client = reqwest::Client::new();
        let resp = client
            .get(&file.url)
            .basic_auth(&acc.username, Some(&acc.password))
            .send()?;

        // only continue if the answer was successfull
        if resp.status() != reqwest::StatusCode::OK {
            bail!("Filer file info failed, please check your credentials or download link");
        }

        // return the result
        Ok(resp)
    }

    /// Prove the downloaded file via the hash
    fn prove_download(&self, file: &DownloadFile, path: &str) -> Result<bool> {
        // Only inspect filer downloads
        if file.hoster != FileHoster::Filer {
            return Ok(false);
        }

        // we dont know the hash alogrithem and can't check
        Ok(true)
    }

    /// Get the next Filer file download id to continue with
    fn get_next_download(&self) -> Result<usize> {
        let qeue = self
            .d_list
            .files_status_hoster(FileStatus::DownloadQueue, FileHoster::Filer)?;

        // check for share-online premium account
        match self.config.get().get_account(
            crate::config::ConfigHoster::Filer,
            crate::config::ConfigAccountStatus::Premium,
        ) {
            Ok(_) => {
                // return a new id when a download id exists
                if !qeue.is_empty() {
                    return Ok(qeue.get(0).ok_or("Id is not available anymore")?.clone());
                }
            }
            Err(_) => {
                let dloads = self
                    .d_list
                    .files_status_hoster(FileStatus::Downloading, FileHoster::Filer)?;

                // start a new free download when nothing is downloaded from so right now
                if dloads.len() == 0 && !qeue.is_empty() {
                    return Ok(qeue.get(0).ok_or("Id is not available anymore")?.clone());
                }
            }
        }

        bail!("No download id availablr for this hoster");
    }
}

impl Filer {
    /// Create a new Share-Online downlaoder
    pub fn new(config: Config, d_list: SmartDownloadList, bus: MessageBus) -> Filer {
        Filer {
            config,
            d_list,
            bus,
        }
    }

    /// Try to get a free download
    fn free_download(&self, file: &DownloadFile) -> Result<::reqwest::Response> {
        let (sender, receiver) = self.bus.channel()?;

        // try to get the chaptchar max 30 times
        for _i in 0..30 {
            // send a request
            sender.send(Message::CaptchaRequest(file.clone()))?;

            // try to get the info for 60 seconds
            let now = ::std::time::Instant::now();
            while now.elapsed() < ::std::time::Duration::from_secs(60) {
                match receiver.recv_timeout(::std::time::Duration::from_secs(5)) {
                    // when value is received and matched reurn download channel
                    Ok(v) => {
                        // we need a captcha response or we continue
                        if let Some(v) = v.get_captcha_response() {
                            if v.id == file.id() && v.file_id == file.file_id {
                                // wait the 30 seconf delay from ShareOnline
                                ::std::thread::sleep(::std::time::Duration::from_secs(30));

                                // create the download stream
                                let resp = reqwest::Client::new().get(&v.url).send()?;

                                // only continue if the answer was successfull
                                if resp.status() != reqwest::StatusCode::OK {
                                    bail!("Share-online free download failed");
                                }

                                // return the result
                                return Ok(resp);
                            }
                        }

                        continue;
                    }
                    // On error either continue or return error
                    Err(e) => {
                        if e == ::std::sync::mpsc::RecvTimeoutError::Timeout {
                            continue;
                        } else {
                            bail!("Can't receive captcha solving");
                        }
                    }
                };
            }
        }

        bail!("Can't do free download");
    }
}
