use error::*;
use std::sync::{Arc, RwLock};
use package::{DownloadPackage, FileStatus};
use shareonline::ShareOnline;
use std::thread;
use writer::FileWriter;
use downloader::Downloader;

/// Download Manager
/// 
/// Shareable struct to access all funtions for the downloads.
#[derive(Debug, Clone)]
pub struct DownloadManager {
    config: DownloadManagerConfig,
    d_list: DownloadList,
    downloader: Downloader,
}

impl DownloadManager {
    /// Create a new Download Manager
    pub fn new(config: DownloadManagerConfig) -> Result<DownloadManager> {
        let d_list = DownloadList::new();

        Ok(DownloadManager {
            config: config.clone(),
            d_list: d_list.clone(),
            downloader: Downloader::new(config, d_list),
        })
    }

    /// Add a new download link to the manager.
    pub fn add_link<S: Into<String>>(&mut self, url: S) -> Result<()> {
        // download the file info
        let fi = self.downloader.check(url.into())?;

        // create a package for the file
        let dp = DownloadPackage::new(fi.name.clone(), vec!(fi));

        // add to links
        self.d_list.add_package(dp);

        Ok(())
    }

    /// Get a copy of the download list
    pub fn get_downloads(&self) -> Result<Vec<DownloadPackage>> {
        self.d_list.get_downloads()
    }

    /// Start the download of an package, by the id
    pub fn start_download(&self, id: usize) -> Result<()> {
        self.d_list.set_status(id, FileStatus::DownloadQueue)
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
            let ids = self.d_list.files_status(FileStatus::DownloadQueue)?;

            // start a new download if its available
            if ids.len() > 0 {
                self.downloader.download(ids.get(0).ok_or("Id is not available anymore")?.clone());
            }

            // end hard if defined
            if self.config.get_status()? == DownloadManagerStatus::EndHard {
                return Ok(());
            }

            // wait to continue the loop
            thread::sleep(::std::time::Duration::from_millis(50));
        }
    }
}

/// Download Manager Config
/// 
/// All the actual configurations of the download manager are saved here.
#[derive(Debug, Clone)]
pub struct DownloadManagerConfig {
    status: Arc<RwLock<DownloadManagerStatus>>,
    max_parallel_downloads: Arc<RwLock<usize>>,
    so_premium_login: Arc<RwLock<Option<(String, String)>>>
}

impl DownloadManagerConfig {
    pub fn new() -> DownloadManagerConfig {
        DownloadManagerConfig {
            status: Arc::new(RwLock::new(DownloadManagerStatus::Running)),
            max_parallel_downloads: Arc::new(RwLock::new(3)),
            so_premium_login: Arc::new(RwLock::new(None)),
        }
    }

    pub fn get_status(&self) -> Result<DownloadManagerStatus> {Ok(self.status.read()?.clone())}
}


#[derive(Debug, Clone, PartialEq)]
pub enum DownloadManagerStatus {
    Running,
    //EndAfterDownloads,
    EndHard
}


/// Download List
/// 
/// Shareable List off all the file informations of the downloads.
#[derive(Debug, Clone)]
pub struct DownloadList {
    downloads: Arc<RwLock<Vec<DownloadPackage>>>,
    changes: Arc<RwLock<Vec<usize>>>,
}

impl DownloadList {
    /// Create a new Download Manager
    pub fn new() -> DownloadList {
        DownloadList {
            downloads: Arc::new(RwLock::new(Vec::new())),
            changes: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Set's the status of an package and all it's childs or a single file by the given id
    pub fn set_status(&self, id: usize, status: FileStatus) -> Result<()> {
        let mut dloads = self.downloads.write()?;

        // check if the id exist for a package
        match dloads.iter().find(|i| i.id() == id) {
            Some(_) => {
                // if yes, then set all childrens to the new status
                dloads.iter_mut().find(|i| i.id() == id).ok_or("The id didn't exist")?.files.iter_mut().for_each(|i| {
                    i.status = status.clone();
                    self.add_changed(i.id());    
                });
            },
            None => {
                // if not, check if a link in a apckage with the id exist and set it's status
                dloads.iter_mut().for_each(|pck| {
                    match pck.files.iter_mut().find(|i| i.id() == id) {
                        Some(i) => {
                            i.status = status.clone();
                            self.add_changed(i.id());
                        },
                        None => {
                            ()
                        }
                    }
                });
            }
        };

        Ok(())
    }

    /// Add a new package to the download list
    pub fn add_package(&self, package: DownloadPackage) -> Result<()> {
        Ok(self.downloads.write()?.push(package))
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

    pub fn get_file(&self, id: &usize) -> Result<::package::DownloadFile> {
        let file = self.downloads.read()?.iter()
                .flat_map(|i| i.files.iter())
                .find(|ref i| id == &i.id()).ok_or("The file can't be found")?.clone();

        Ok(file.clone())
    }

    pub fn add_changed(&self, id: usize) -> Result<()> {
        Ok(self.changes.write()?.push(id))
    }

    pub fn delete_changes(&self) -> Result<Vec<usize>> {
        let tmp = self.changes.read()?.clone();
        self.changes.write()?.retain(|_| false);
        Ok(tmp)
    }
}