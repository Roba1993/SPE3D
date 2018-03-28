//! Share-Online downloader. Responsible to download files from Share-Online and
//! checking the status of a download link.
//!
//! Right now only Premium Accounts are supported.

use error::*;
use reqwest;
use regex::Regex;
use models::{DownloadFile, FileHash, FileStatus, FileHoster};
use std::collections::HashMap;
use config::Config;

#[derive(Debug, Clone)]
pub struct ShareOnline {
    config: Config,
    expire_date: String,
    key: String,
}

impl ShareOnline {
    pub fn new (config: Config) -> ShareOnline {
        ShareOnline {
            config,
            expire_date: "".into(),
            key: "".into()
        }
    }

    pub fn login(&mut self) -> Result<()> {
        let usr = self.config.get().share_online.ok_or("No share-online logins defined")?.username;
        let pwd = self.config.get().share_online.ok_or("No share-online logins defined")?.password;

        // download the user data
        let login_url = format!("http://api.share-online.biz/account.php?username={}&password={}&act=userDetails", usr, pwd);
        let mut resp = reqwest::get(&login_url)?;

        // only continue if the answer was successfull
        if resp.status() != reqwest::StatusCode::Ok {
            bail!("Share-online login failed, please check your credentials");
        }

        // get the body
        let body = resp.text()?;

        // extract the user key and expire date
        self.key = String::from(&Regex::new(r"a=(\w+)")?.find(&body).ok_or("No user key available")?.as_str()[2..]);
        self.expire_date = String::from(&Regex::new(r"expire_date=(\w+)")?.find(&body).ok_or("No expire date available")?.as_str()[12..]);

        Ok(())
    }

    pub fn download_link<S: Into<String>>(&mut self, url: S) -> Result<reqwest::Response> {
        // download the header data
        let link = self.download_file_info(url)?;

        // start the real download
        self.download_file(&link)
    }

    pub fn check<S: Into<String>>(&self, url: S) -> Result<Option<DownloadFile>> {
        self.download_file_info(url).map(Some)
    }

    pub fn download_file_info<S: Into<String>>(&self, url: S) -> Result<DownloadFile> {
        // split the url and id
        let url = url.into();
        let res: Vec<&str> = url.split("/dl/").collect();

        // check if the values are valid
        let link : &str = res.get(0).ok_or("Can't find the share-online host in the link")?;
        let id = res.get(1).ok_or("Can't find a download id in the link")?;

        // check if the right host is set
        if link != "http://www.share-online.biz" && link != "https://www.share-online.biz" {
            bail!("The given link wasn't a share-online download link");
        }
        
        let usr = self.config.get().share_online.ok_or("No share-online logins defined")?.username;
        let pwd = self.config.get().share_online.ok_or("No share-online logins defined")?.password;

        // make the request call
        let info_url = format!("http://api.share-online.biz/account.php?username={}&password={}&act=download&lid={}", usr, pwd, id);
        let mut resp = reqwest::get(&info_url)?;

        // only continue if the answer was successfull
        if resp.status() != reqwest::StatusCode::Ok {
            bail!("Share-online file info failed, please check your credentials or download link");
        }

        // get the body
        let body = resp.text()?;

        let status = String::from(&Regex::new(r"STATUS: ([^\s]+)")?.find(&body).ok_or("No status available")?.as_str()[8..]);
        println!("STATUS: {}", status);

        // define the addiotional infos with the premium url
        let mut infos = HashMap::new();
        infos.insert("premium_url".to_string(), String::from(&Regex::new(r"URL: ([^\s]+)")?.find(&body).ok_or("No url available")?.as_str()[5..]));

        let mut f_info = DownloadFile::new();
        // todo check the file status correctly
        f_info.status = FileStatus::Online;
        f_info.host = FileHoster::ShareOnline;
        f_info.name = String::from(&Regex::new(r"NAME: ([^\s]+)")?.find(&body).ok_or("No name available")?.as_str()[6..]);
        f_info.url = url.clone();
        f_info.size = Regex::new(r"SIZE: ([^\s]+)")?.find(&body).ok_or("No size available")?.as_str()[6..].parse::<usize>()?;
        f_info.hash = FileHash::Md5(String::from(&Regex::new(r"MD5: ([^\s]+)")?.find(&body).ok_or("No md5 available")?.as_str()[5..]));
        f_info.infos = infos;

        Ok(f_info)
    }

    pub fn download_file(&mut self, link: &DownloadFile) -> Result<reqwest::Response> {
        self.login()?;
        let link = self.download_file_info(link.url.clone())?;

        // set the download header
        let mut headers = reqwest::header::Headers::new();
        headers.set_raw("Cookie", format!("a={}; Expires={}; HttpOnly; Path=/; Domain=.share-online.biz", self.key, self.expire_date));

        // get the premium url
        let url = link.infos.get("premium_url").ok_or("Premium url not found")?;

        // make the request
        let resp = reqwest::Client::new().get(url)
            .headers(headers)
            .send()?;

        // only continue if the answer was successfull
        if resp.status() != reqwest::StatusCode::Ok {
            bail!("Share-online file info failed, please check your credentials or download link");
        }

        // return the result
        Ok(resp)
    }
}