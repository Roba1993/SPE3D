//! This create provides the functionality of a download manager as a crate functionality.
//!
//! It serves the use-cases for the [spe3d download manager](https://github.com/Roba1993/SPE3D).
//!

#[macro_use] extern crate error_chain;
#[macro_use]extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate toml;
extern crate dlc_decrypter;
extern crate reqwest;
extern crate regex;
extern crate md5;

pub mod error;
pub mod config;
pub mod models;
pub mod loader;

// reexports for easier use of the important structures
pub use config::Config;

// Imports for the Download Manager
use error::*;
use error_chain::ChainedError;
use models::{DownloadPackage, FileStatus, SmartDownloadList};
use std::thread;
use loader::Downloader;
use dlc_decrypter::DlcDecoder;
use config::ConfigData;

/// Main entry point for the API. This structure allows to add potential downloads
/// to a shared list and gives the ability to start this downloads.
#[derive(Clone)]
pub struct DownloadManager {
    config: Config,
    d_list: SmartDownloadList,
    downloader: Downloader,
}

impl DownloadManager {
    /// Create a new Download Manager based on a configuration
    pub fn new(config: Config) -> Result<DownloadManager> {
        let d_list = SmartDownloadList::new();

        Ok(DownloadManager {
            config: config.clone(),
            d_list: d_list.clone(),
            downloader: Downloader::new(config, d_list),
        })
    }

    /// Add one or multible links to the manager as a new download package
    pub fn add_links<S: Into<String>>(&self, name: S, urls: Vec<String>) -> Result<()> {
        // download the file info
        let f_infos = urls.into_iter().map(|u| self.downloader.check(u)).filter(|u| u.is_ok()).map(|u| u.unwrap()).collect();

        // create a package for the file
        let dp = DownloadPackage::new(name.into(), f_infos);

        // add to links
        self.d_list.add_package(dp)
    }

    /// Add a new package to the download manager
    pub fn add_package<P: Into<DownloadPackage>>(&self, pck: P) -> Result<()> {
        let mut pck = pck.into();
        pck.files = pck.files.into_iter().map(|f| self.downloader.check(f.url)).filter(|u| u.is_ok()).map(|u| u.unwrap()).collect();
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
    pub fn recv_update(&self) -> Result<Vec<DownloadPackage>> {
        self.d_list.recv_update()
    }

    /// Get the actual configuration
    pub fn get_config(&self) -> Config {
        self.config.clone()
    }

    /********************* Private Functions *****************/
    /// The internal loop which runs all 0.5 seconds
    fn internal_loop(&self) -> Result<()> {
        // get all download id's in queue to start
        let qeue = self.d_list.files_status(FileStatus::DownloadQueue)?;
        let dloads = self.d_list.files_status(FileStatus::Downloading)?;

        // start a new download if its available
        if dloads.len() < 3 && !qeue.is_empty() {
            self.downloader.download(qeue.get(0).ok_or("Id is not available anymore")?.clone());
        }

        // wait to continue the loop
        thread::sleep(::std::time::Duration::from_millis(500));
        Ok(())
    }
}


