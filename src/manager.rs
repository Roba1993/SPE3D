use error::*;
use std::sync::{Arc, RwLock};
use package::{DownloadPackage, FileStatus};
use shareonline::ShareOnline;
use std::thread;
use downloader::FileWriter;


#[derive(Debug, Clone)]
pub struct DownloadManager {
    status: Arc<RwLock<DownloadManagerStatus>>,
    settings: Arc<RwLock<DownloadManagerSettings>>,
    so: Arc<RwLock<ShareOnline>>,
    downloads: DownloadList
}

impl DownloadManager {
    /// Create a new Download Manager
    pub fn new<S: Into<String>>(usr: S, pwd: S) -> Result<DownloadManager> {
        Ok(DownloadManager {
            status: Arc::new(RwLock::new(DownloadManagerStatus::Running)),
            settings: Arc::new(RwLock::new(DownloadManagerSettings::new())),
            so: Arc::new(RwLock::new(ShareOnline::new(usr, pwd)?)),
            downloads: DownloadList::new()
        })
    }

    /// Add a new download link to the manager.
    pub fn add_link<S: Into<String>>(&mut self, url: S) -> Result<()> {
        // download the file info
        let fi = self.so.read()?.download_file_info(url.into())?;

        // create a package for the file
        let dp = DownloadPackage::new(fi.name.clone(), vec!(fi));

        // add to links
        self.downloads.add_package(dp);

        Ok(())
    }

    /// Get a copy of the download list
    pub fn get_downloads(&self) -> Result<Vec<DownloadPackage>> {
        self.downloads.get_downloads()
    }

    /// Start the download of an package, by the id
    pub fn start_download(&self, id: usize) -> Result<()> {
        self.downloads.set_status(id, FileStatus::DownloadQueue)
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
                match dm.run() {
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
    fn run(&self) -> Result<()> {
        loop {
            // get the actual local settings as a copy
            let settings = self.settings.read()?.clone();

            // get all download id's in queue to start
            let ids = self.downloads.files_status(FileStatus::DownloadQueue)?;

            // start a new download if its available
            if ids.len() > 0 {
                let this = self.clone();         
                thread::spawn(move || {
                    this.start_download_process(ids.get(0).unwrap()).unwrap();
                });
            }

            // end hard if defined
            if self.status.read()?.clone() == DownloadManagerStatus::EndHard {
                return Ok(());
            }

            // wait to continue the loop
            thread::sleep(::std::time::Duration::from_millis(50));
        }
    }

    fn start_download_process(&self, id: &usize) -> Result<()> {
        // define the stream object this thread executes
        let mut stream;
        
        // create the stream
        {
            self.downloads.set_status(id.clone(), FileStatus::Downloading)?;

            let file = self.downloads.get_file(id)?;
            stream = self.so.write()?.download_file(file)?;
        }
        
        // start the download
        let hash = stream.write_to_file("./out/new.txt")?;
        println!("HASH FROM DLOAD: {}", hash);
        
        // check the download and update the status
        {
            let file = self.downloads.get_file(id)?;

            // check if the hash matched
            if hash == file.hash.md5().ok_or("No MD5 hash available")? {
                self.downloads.set_status(id.clone(), FileStatus::Downloaded);
            }
            else {
                self.downloads.set_status(id.clone(), FileStatus::WrongHash);
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DownloadManagerStatus {
    Running,
    //EndAfterDownloads,
    EndHard
}

#[derive(Debug, Clone)]
pub struct DownloadManagerSettings {
    max_parallel_downloads: usize
}

impl DownloadManagerSettings {
    pub fn new() -> DownloadManagerSettings {
        DownloadManagerSettings {
            max_parallel_downloads: 3,
        }
    }
}

#[derive(Debug, Clone)]
struct DownloadList {
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

    pub fn add_changed(&self, id: usize) -> Result<()> {
        Ok(self.changes.write()?.push(id))
    }

    pub fn delete_changes(&self) -> Result<Vec<usize>> {
        let tmp = self.changes.read()?.clone();
        self.changes.write()?.retain(|_| false);
        Ok(tmp)
    }

    pub fn get_file(&self, id: &usize) -> Result<::package::DownloadFile> {
        let file = self.downloads.read()?.iter()
                .flat_map(|i| i.files.iter())
                .find(|ref i| id == &i.id()).ok_or("The can't be found")?.clone();

        Ok(file.clone())
    }
}