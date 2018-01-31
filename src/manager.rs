use error::*;
use std::sync::{Arc, RwLock};
use package::{DownloadPackage, FileStatus};
use std::thread;
use downloader::Downloader;
use ws;
use serde_json;
use serde::Serialize;
use config::Config;

/// Download Manager
/// 
/// Shareable struct to access all funtions for the downloads.
#[derive(Clone)]
pub struct DownloadManager {
    config: Config,
    d_list: DownloadList,
    downloader: Downloader,
}

impl DownloadManager {
    /// Create a new Download Manager
    pub fn new(config: Config) -> Result<DownloadManager> {
        let d_list = DownloadList::new();

        Ok(DownloadManager {
            config: config.clone(),
            d_list: d_list.clone(),
            downloader: Downloader::new(config, d_list),
        })
    }

    /// Add a new download link to the manager.
    pub fn add_links<S: Into<String>>(&self, name: S, urls: Vec<String>) -> Result<()> {
        // download the file info
        let f_infos = urls.into_iter().map(|u| self.downloader.check(u)).filter(|u| u.is_ok()).map(|u| u.unwrap()).collect();

        // create a package for the file
        let dp = DownloadPackage::new(name.into(), f_infos);

        // add to links
        self.d_list.add_package(dp)
    }

    pub fn add_package<P: Into<DownloadPackage>>(&self, pck: P) -> Result<()> {
        let mut pck = pck.into();
        pck.files = pck.files.into_iter().map(|f| self.downloader.check(f.url)).filter(|u| u.is_ok()).map(|u| u.unwrap()).collect();
        self.d_list.add_package(pck)
    }

    /// Get a copy of the download list
    pub fn get_downloads(&self) -> Result<Vec<DownloadPackage>> {
        self.d_list.get_downloads()
    }

    /// Start the download of an package, by the id
    pub fn start_download(&self, id: usize) -> Result<()> {
        self.d_list.set_status(id, FileStatus::DownloadQueue)
    }

    /// Set a websocket sender to broadcast the changed id's
    pub fn set_ws_sender(&self, sender: ws::Sender) -> Result<()> {
        self.d_list.set_ws_sender(sender)
    }

    // start the download manager
    pub fn start(&self) -> thread::JoinHandle<()> {
        // clone the download manager to use it in the thread
        let dm = self.clone();

        // spawn a new thread with an andless loop
        thread::spawn(move || {
            // counter for failures happend in a row
            let mut failures = 0;

            // run until end or failures reached maximum level
            loop {
                // run the logic
                match dm.internal_loop() {
                    Ok(_) => {
                        break;
                    },
                    Err(_) => {
                        // exit when we reached max number of errors
                        failures += failures + 1;
                        if failures > 10 {
                            break;
                        }
                    }
                }
            }
        })
    }

    /********************* Private Functions *****************/
    fn internal_loop(&self) -> Result<()> {
        loop {
            // get all download id's in queue to start
            let qeue = self.d_list.files_status(FileStatus::DownloadQueue)?;
            let dloads = self.d_list.files_status(FileStatus::Downloading)?;

            // start a new download if its available
            if dloads.len() < 3 && qeue.len() > 0 {
                self.downloader.download(qeue.get(0).ok_or("Id is not available anymore")?.clone());
            }

            // wait to continue the loop
            thread::sleep(::std::time::Duration::from_millis(50));
        }
    }
}


/// Download List
/// 
/// Shareable List off all the file informations of the downloads.
#[derive(Clone)]
pub struct DownloadList {
    downloads: Arc<RwLock<Vec<DownloadPackage>>>,
    ws_sender: Arc<RwLock<Option<ws::Sender>>>,
}

impl DownloadList {
    /// Create a new Download Manager
    pub fn new() -> DownloadList {
        DownloadList {
            downloads: Arc::new(RwLock::new(Vec::new())),
            ws_sender: Arc::new(RwLock::new(None)),
        }
    }

    /// Set's the status of an package and all it's childs or a single file by the given id
    pub fn set_status(&self, id: usize, status: FileStatus) -> Result<()> {
        {
            let mut dloads = self.downloads.write()?;

            // check if the id exist for a package
            match dloads.iter().find(|i| i.id() == id) {
                Some(_) => {
                    // if yes, then set all childrens to the new status
                    dloads.iter_mut().find(|i| i.id() == id).ok_or("The id didn't exist")?.files.iter_mut().for_each(|i| {
                        i.status = status.clone();
                    });
                },
                None => {
                    // if not, check if a link in a apckage with the id exist and set it's status
                    dloads.iter_mut().for_each(|pck| {
                        match pck.files.iter_mut().find(|i| i.id() == id) {
                            Some(i) => {
                                i.status = status.clone(); 
                            },
                            None => {
                                ()
                            }
                        }
                    });
                }
            };
        }

        self.ws_send_change()?;
        Ok(())
    }

    pub fn set_downloaded(&self, id: usize, size: usize) -> Result<()> {
        self.downloads.write()?.iter_mut().for_each(|pck| 
            match pck.files.iter_mut().find(|f| f.id() == id) {
                Some(i) => i.downloaded = size,
                None => ()
            }
        );

        self.ws_send_change()?;
        Ok(())
    }

    /// Add a new package to the download list
    pub fn add_package(&self, package: DownloadPackage) -> Result<()> {
        self.downloads.write()?.push(package);
        self.ws_send_change()
    }

    /// Get a copy of the download list
    pub fn get_downloads(&self) -> Result<Vec<DownloadPackage>> {
        Ok(self.downloads.read()?.clone())
    }

    /// Gives a list of the files with the status back
    pub fn files_status(&self, status: FileStatus) -> Result<Vec<usize>> {
        // get all download id's in queue to start
        let ids = self.downloads.read()?.iter().map(|pck|
                pck.files.iter()
                .filter(|i| i.status == status)
                .map(|i| i.id()).collect::<Vec<usize>>()
            ).flat_map(|i| i.into_iter())
            .collect::<Vec<usize>>();

        Ok(ids)
    }

    /// Returns a copy of the file info
    pub fn get_file(&self, id: &usize) -> Result<::package::DownloadFile> {
        let file = self.downloads.read()?.iter()
                .flat_map(|i| i.files.iter())
                .find(|ref i| id == &i.id()).ok_or("The file can't be found")?.clone();

        Ok(file.clone())
    }

    /// Get a package by it's id or it's child id
    pub fn get_package(&self, id: &usize) -> Result<DownloadPackage> {
        match self.downloads.read()?.iter().find(|i| &i.id() == id) {
            Some(i) => Ok(i.clone()),
            None => Ok(self.downloads.read()?.iter().find(|i| i.files.iter().any(|j| &j.id() == id)).ok_or("No download package available")?.clone())
        }
    }

    /// Add a websocket sender to broadcast the changed id's
    pub fn set_ws_sender(&self, sender: ws::Sender) -> Result<()> {
        let mut s = self.ws_sender.write()?;
        *s = Some(sender);
        Ok(())
    }

    /// Send the actual status of the file info for the given id to all
    /// connected websocket clients
    pub fn ws_send_change(&self) -> Result<()> {
        let f_info = self.get_downloads()?;
        self.ws_send(f_info)
    }

    /// Send the mesage to all connected websocket clients
    fn ws_send<T: Serialize>(&self, msg: T) -> Result<()> {
        let msg = serde_json::to_string(&msg)?;

        match self.ws_sender.read()?.as_ref() {
            Some(s) => {
                s.broadcast(msg)?;
            },
            None => {}
        };

        Ok(())
    }
}