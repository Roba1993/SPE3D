//! Share-Online downloader. Responsible to download files from Share-Online and
//! checking the status of a download link.
//!
//! Right now only Premium Accounts are supported.

use crate::bus::{Message, MessageBus};
use crate::config::Config;
use crate::error::*;
use crate::loader::Loader;
use md5::{Digest, Md5};
use crate::models::{DownloadFile, FileHash, FileHoster, FileStatus, SmartDownloadList};
use regex::Regex;
use reqwest;
use std::fs::File;

/// Share-Online downloader struct which allows to download files from share-online with an premium account.
#[derive(Clone)]
pub struct ShareOnline {
    config: Config,
    d_list: SmartDownloadList,
    bus: MessageBus,
}

impl Loader for ShareOnline {
    /// This function updates an Share-Online account with the actual status
    fn update_account(&self, account: &mut crate::config::ConfigAccount) -> Result<()> {
        // This implementation can only check share-online accounts
        if account.hoster != crate::config::ConfigHoster::ShareOnline {
            bail!("Not a Share-Online Account");
        }

        let url = format!(
            "https://api.share-online.biz/account.php?username={}&password={}&act=userDetails",
            account.username, account.password
        );
        let mut resp = reqwest::get(&url)?;

        // only continue if the answer was successfull
        if resp.status() != reqwest::StatusCode::Ok {
            bail!("Share-online login failed, some connection error occurred");
        }

        // get the body
        let body = resp.text()?;

        // when the account isn't valid
        if body == "** INVALID USER DATA **" {
            account.status = crate::config::ConfigAccountStatus::NotValid;
            return Ok(());
        }

        // get the account type
        let acc_type = &Regex::new(r"group=(\w+)")?
            .find(&body)
            .ok_or("No account group available")?
            .as_str()[6..];
        if acc_type == "Sammler" {
            account.status = crate::config::ConfigAccountStatus::Free;
            return Ok(());
        } else if acc_type == "Premium" {
            account.status = crate::config::ConfigAccountStatus::Premium;
            return Ok(());
        }

        println!("Unkown Account type: {}", acc_type);
        Ok(())
    }

    /// Check the download url and return the file info
    fn check_url(&self, url: &str) -> Result<Option<DownloadFile>> {
        if !url.starts_with("https://www.share-online.biz")
            && !url.starts_with("http://www.share-online.biz")
            && !url.starts_with("www.share-online.biz")
        {
            bail!("The given link wasn't a share-online download link");
        }

        let req = format!(
            "https://api.share-online.biz/linkcheck.php?md5=1&links={}",
            url
        );
        let mut resp = reqwest::get(&req)?;

        // only continue if the answer was successfull
        if resp.status() != reqwest::StatusCode::Ok {
            bail!("Share-online login failed, some connection error occurred");
        }

        // get the body and split the data
        let body = resp.text()?;
        let res: Vec<&str> = body.trim().split(";").collect();

        // extract the data and save it as a DownloadFile
        let mut file = DownloadFile::default();
        file.hoster = FileHoster::ShareOnline;
        file.file_id = res.get(0).ok_or("Can't get file id")?.to_string();
        file.url = format!("https://www.share-online.biz/dl/{}", file.file_id);
        file.status = if res.get(1).ok_or("Can't get file status")? == &"OK" {
            FileStatus::Online
        } else {
            FileStatus::Offline
        };
        file.name = res.get(2).ok_or("Can't get file name")?.to_string();
        file.size = res.get(3).ok_or("Can't get file size")?.parse()?;
        file.hash = FileHash::Md5(res.get(4).ok_or("Can't get file hash")?.to_string());

        Ok(Some(file))
    }

    /// Download a file, with this laoder
    fn download(&self, file: &DownloadFile) -> Result<::reqwest::Response> {
        let acc = match self.config.get().get_account(
            crate::config::ConfigHoster::ShareOnline,
            crate::config::ConfigAccountStatus::Premium,
        ) {
            Ok(a) => a,
            Err(_) => {
                return self.free_download(file);
            }
        };

        let (key, expire_date) = self.login(acc)?;
        let url = self.get_dload_url(file)?;

        // set the download header
        let mut headers = reqwest::header::Headers::new();
        headers.set_raw(
            "Cookie",
            format!(
                "a={}; Expires={}; HttpOnly; Path=/; Domain=.share-online.biz",
                key, expire_date
            ),
        );

        // make the request
        let resp = reqwest::Client::new().get(&url).headers(headers).send()?;

        // only continue if the answer was successfull
        if resp.status() != reqwest::StatusCode::Ok {
            bail!("Share-online file info failed, please check your credentials or download link");
        }

        // return the result
        Ok(resp)
    }

    /// Prove the downloaded file via the hash
    fn prove_download(&self, file: &DownloadFile, path: &str) -> Result<bool> {
        // Only inspect share online downloads
        if file.hoster != FileHoster::ShareOnline {
            return Ok(false);
        }

        // hash the file
        let hash = Md5::digest_reader(&mut File::open(path)?)?;
        let hash = format!("{:x}", hash);

        // check the hash
        if hash == file.hash.md5().ok_or("No MD5 hash available")? {
            return Ok(true);
        }

        bail!("Download is incorrect, hash is not matching");
    }

    /// Get the next ShareOnline file download id to continue with
    fn get_next_download(&self) -> Result<usize> {
        let qeue = self
            .d_list
            .files_status_hoster(FileStatus::DownloadQueue, FileHoster::ShareOnline)?;

        // check for share-online premium account
        match self.config.get().get_account(
            crate::config::ConfigHoster::ShareOnline,
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
                    .files_status_hoster(FileStatus::Downloading, FileHoster::ShareOnline)?;

                // start a new free download when nothing is downloaded from so right now
                if dloads.len() == 0 && !qeue.is_empty() {
                    return Ok(qeue.get(0).ok_or("Id is not available anymore")?.clone());
                }
            }
        }

        bail!("No download id availablr for this hoster");
    }
}

impl ShareOnline {
    /// Create a new Share-Online downlaoder
    pub fn new(config: Config, d_list: SmartDownloadList, bus: MessageBus) -> ShareOnline {
        ShareOnline {
            config,
            d_list,
            bus,
        }
    }

    /// Share-Online premium login
    fn login(&self, acc: crate::config::ConfigAccount) -> Result<(String, String)> {
        // download the user data
        let login_url = format!(
            "https://api.share-online.biz/account.php?username={}&password={}&act=userDetails",
            acc.username, acc.password
        );
        let mut resp = reqwest::get(&login_url)?;

        // only continue if the answer was successfull
        if resp.status() != reqwest::StatusCode::Ok {
            bail!("Share-online login failed, please check your credentials");
        }

        // get the body
        let body = resp.text()?;

        // extract the user key and expire date
        let key = String::from(
            &Regex::new(r"a=(\w+)")?
                .find(&body)
                .ok_or("No user key available")?
                .as_str()[2..],
        );
        let expire_date = String::from(
            &Regex::new(r"expire_date=(\w+)")?
                .find(&body)
                .ok_or("No expire date available")?
                .as_str()[12..],
        );

        Ok((key, expire_date))
    }

    /// Get the premium download url
    fn get_dload_url(&self, file: &DownloadFile) -> Result<String> {
        let acc = self.config.get().get_account(
            crate::config::ConfigHoster::ShareOnline,
            crate::config::ConfigAccountStatus::Premium,
        )?;

        // make the request call
        let info_url = format!(
            "https://api.share-online.biz/account.php?username={}&password={}&act=download&lid={}",
            acc.username, acc.password, file.file_id
        );
        let mut resp = reqwest::get(&info_url)?;

        // only continue if the answer was successfull
        if resp.status() != reqwest::StatusCode::Ok {
            bail!("Share-online file info failed, please check your credentials or download link");
        }

        // get the body
        let body = resp.text()?;

        // check if the file status is still okay
        if &Regex::new(r"STATUS: ([^\s]+)")?
            .find(&body)
            .ok_or("No status available")?
            .as_str()[8..]
            != "online"
        {
            bail!("The file is not online anymore");
        }

        // check if the hash of the both files are still matching
        if Regex::new(r"MD5: ([^\s]+)")?
            .find(&body)
            .ok_or("No md5 available")?
            .as_str()[5..]
            != file.hash.md5().ok_or("FileInfo has no hash")?
        {
            bail!("The Hash of the file to download don't match anymore")
        }

        // Return the premium download url
        Ok(String::from(
            &Regex::new(r"URL: ([^\s]+)")?
                .find(&body)
                .ok_or("No url available")?
                .as_str()[5..],
        ))
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
                                if resp.status() != reqwest::StatusCode::Ok {
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
