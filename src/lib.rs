#![recursion_limit = "128"]

//! This create provides the functionality of a download manager as a crate functionality.
//!
//! It serves the use-cases for the [spe3d download manager](https://github.com/Roba1993/SPE3D).
//!

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;
extern crate dlc_decrypter;
extern crate jd_decrypter;
extern crate md5;
extern crate regex;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate toml;

pub mod bus;
pub mod config;
pub mod error;
pub mod loader;
pub mod models;

// reexports for easier use of the important structures
pub use crate::config::Config;

// Imports for the Download Manager
use crate::bus::MessageBus;
use crate::error::*;
use crate::loader::Downloader;
use crate::models::{CaptchaResult, DownloadPackage, FileStatus, SmartDownloadList};
use dlc_decrypter::DlcDecoder;
use error_chain::ChainedError;
use std::thread;

/// Main entry point for the API. This structure allows to add potential downloads
/// to a shared list and gives the ability to start this downloads.
#[derive(Clone)]
pub struct DownloadManager {
    config: Config,
    bus: MessageBus,
    d_list: SmartDownloadList,
    downloader: Downloader,
}

impl DownloadManager {
    /// Create a new Download Manager based on a configuration
    pub fn new(config: Config) -> Result<DownloadManager> {
        let bus = MessageBus::new();
        let d_list = SmartDownloadList::new(&bus)?;

        Ok(DownloadManager {
            config: config.clone(),
            bus: bus.clone(),
            d_list: d_list.clone(),
            downloader: Downloader::new(config, d_list, bus),
        })
    }

    /// Add one or multible links to the manager as a new download package
    pub fn add_links<S: Into<String>>(&self, name: S, urls: Vec<String>) -> Result<()> {
        // download the file info
        let f_infos = urls
            .into_iter()
            .map(|u| self.downloader.check(u))
            .filter(|u| u.is_ok())
            .map(|u| u.unwrap())
            .collect();

        // create a package for the file
        let dp = DownloadPackage::new(name.into(), f_infos);

        // add to links
        self.d_list.add_package(dp)
    }

    /// Add a new package to the download manager
    pub fn add_package<P: Into<DownloadPackage>>(&self, pck: P) -> Result<()> {
        let mut pck = pck.into();
        pck.files = pck
            .files
            .into_iter()
            .map(|f| self.downloader.check(f.url))
            .filter(|u| u.is_ok())
            .map(|u| u.unwrap())
            .collect();
        self.d_list.add_package(pck)
    }

    /// Add a new dlc dataset to the download manager
    pub fn add_dlc(&self, data: &str) -> Result<()> {
        // extract the dlc package
        let dlc = DlcDecoder::new();
        let pck = dlc.from_data(data.as_bytes())?;
        self.add_package(pck)?;
        Ok(())
    }

    /// Remove a download file or a package from the manager
    pub fn remove(&self, id: usize) -> Result<()> {
        self.d_list.remove(id)
    }

    /// Get a copy of the download list
    pub fn get_downloads(&self) -> Result<Vec<DownloadPackage>> {
        self.d_list.get_downloads()
    }

    /// Start the download of an package, by the id
    pub fn start_download(&self, id: usize) -> Result<()> {
        self.d_list.set_status(id, &FileStatus::DownloadQueue)
    }

    /// start the download manager itself
    pub fn start(&self) -> thread::JoinHandle<()> {
        // clone the download manager to use it in the thread
        let dm = self.clone();

        // spawn a new thread with an andless loop
        thread::spawn(move || {
            // check all accounts initially
            for acc in dm.config.get().accounts {
                let _ = dm.check_account(acc.id);
            }

            // run until end or failures reached maximum level
            loop {
                // run the logic
                if let Err(e) = dm.internal_loop() {
                    println!("{}", e.display_chain().to_string());
                }
            }
        })
    }

    /// Receive a new download list on each change
    pub fn get_bus(&self) -> MessageBus {
        self.bus.clone()
    }

    /// Get the actual configuration
    pub fn get_config(&self) -> Config {
        self.config.clone()
    }

    /// Check the actual status of an account and return it
    pub fn check_account(&self, id: usize) -> Result<crate::config::ConfigAccount> {
        for mut acc in self.config.get().accounts {
            if acc.id == id {
                self.downloader.update_account(&mut acc)?;
                self.config.update_account(acc.clone())?;
                return Ok(acc);
            }
        }

        bail!("No account found with the given id");
    }

    /// Add an captcha result to the download manager
    pub fn add_captcha_result(&self, cr: CaptchaResult) -> Result<()> {
        let sender = self.bus.get_sender()?;
        sender.send(crate::bus::Message::CaptchaResponse(cr))?;
        Ok(())
    }

    /********************* Private Functions *****************/
    /// The internal loop which runs all 0.2 seconds
    fn internal_loop(&self) -> Result<()> {
        // get all id's in queue which are downloading right now
        let dloads = self.d_list.files_status(FileStatus::Downloading)?;

        // when less than 3 downloads happen and a new download id is available
        if dloads.len() < 3 {
            if let Ok(id) = self.downloader.get_next_download() {
                self.downloader.download(id);
            }
        }

        // check always one account - each account every 5 minutes
        // on time error also check the account
        for acc in self.config.get().accounts {
            if let Ok(e) = acc.checked.elapsed() {
                if e > ::std::time::Duration::from_secs(300)
                    || acc.status == self::config::ConfigAccountStatus::Unknown
                {
                    self.check_account(acc.id)?;
                    break;
                }
            } else {
                self.check_account(acc.id)?;
                break;
            }
        }

        // wait to continue the loop
        thread::sleep(::std::time::Duration::from_millis(200));
        Ok(())
    }
}
