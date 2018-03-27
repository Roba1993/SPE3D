use error::*;
use std::sync::{Arc, RwLock, Mutex};
use package::{DownloadPackage, FileStatus};
use std::thread;
use downloader::Downloader;
use config::Config;
use std::fs::File;
use std::io::prelude::*;
use package;
use dlc_decrypter::DlcDecoder;
use std::sync::mpsc::{Sender, Receiver, channel};

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

    pub fn add_dlc(&self, data: &str) -> Result<()> {
        // extract the dlc package
        let dlc = DlcDecoder::new();
        let pck = dlc.from_data(data.as_bytes())?;
        self.add_package(pck)?;
        Ok(())
    }

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
                        failures += 1;
                        if failures > 10 {
                            break;
                        }
                    }
                }
            }
        })
    }

    pub fn recv_update(&self) -> Result<Vec<DownloadPackage>> {
        self.d_list.recv_update()
    }

    /********************* Private Functions *****************/
    fn internal_loop(&self) -> Result<()> {
        loop {
            // get all download id's in queue to start
            let qeue = self.d_list.files_status(FileStatus::DownloadQueue)?;
            let dloads = self.d_list.files_status(FileStatus::Downloading)?;

            // start a new download if its available
            if dloads.len() < 3 && !qeue.is_empty() {
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
    sender: Arc<Mutex<Sender<Vec<DownloadPackage>>>>,
    receiver: Arc<Mutex<Receiver<Vec<DownloadPackage>>>>,
}

impl DownloadList {
    /// Create a new Download Manager
    pub fn new() -> DownloadList {
        let (sender, receiver) = channel();

        // create the download list
        let d_list = DownloadList {
            downloads: Arc::new(RwLock::new(Vec::new())),
            sender: Arc::new(Mutex::new(sender)),
            receiver: Arc::new(Mutex::new(receiver))
        };

        // load the previous state from file
        match d_list.load() {
            Ok(_) => {},
            Err(_) => {println!("Can't read previus status of the download list")}
        };

        // return the download list
        d_list
    }

    /// Set's the status of an package and all it's childs or a single file by the given id
    pub fn set_status(&self, id: usize, status: &FileStatus) -> Result<()> {
        {
            let mut dloads = self.downloads.write()?;

            // check if the id exist for a package
            match dloads.iter().find(|i| i.id() == id) {
                Some(_) => {
                    // if yes, then set all childrens to the new status
                    dloads.iter_mut().find(|i| i.id() == id).ok_or("The id didn't exist")?.files.iter_mut().for_each(|i| {
                        i.status = *status;
                    });
                },
                None => {
                    // if not, check if a link in a apckage with the id exist and set it's status
                    dloads.iter_mut().for_each(|pck| {
                        if let Some(i) = pck.files.iter_mut().find(|i| i.id() == id) {
                            i.status = *status; 
                        }
                    });
                }
            };
        }

        self.ws_send_change()?;
        self.save()
    }

    pub fn add_downloaded(&self, id: usize, size: usize) -> Result<()> {
        self.downloads.write()?.iter_mut().for_each(|pck| 
            if let Some(i) = pck.files.iter_mut().find(|f| f.id() == id) { 
                i.downloaded += size;
                i.speed = size;
            }
        );

        self.ws_send_change()?;
        Ok(())
    }

    pub fn set_downloaded(&self, id: usize, size: usize) -> Result<()> {
        self.downloads.write()?.iter_mut().for_each(|pck| 
            if let Some(i) = pck.files.iter_mut().find(|f| f.id() == id) { 
                i.downloaded = size;
            }
        );

        self.ws_send_change()?;
        Ok(())
    }

    /// Add a new package to the download list
    pub fn add_package(&self, package: DownloadPackage) -> Result<()> {
        self.downloads.write()?.push(package);
        self.ws_send_change()?;
        self.save()
    }

    /// Add a new package to the download list
    pub fn remove(&self, id: usize) -> Result<()> {
        // delete a children
        self.downloads.write()?.iter_mut().for_each(|p| p.files.retain(|i| i.id() != id));
        // delete a container also all empty containers
        self.downloads.write()?.retain(|i| i.id() != id && !i.files.is_empty() );

        self.ws_send_change()?;
        self.save()
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
                .find(|i| id == &i.id()).ok_or("The file can't be found")?.clone();

        Ok(file.clone())
    }

    /// Get a package by it's id or it's child id
    pub fn get_package(&self, id: &usize) -> Result<DownloadPackage> {
        match self.downloads.read()?.iter().find(|i| &i.id() == id) {
            Some(i) => Ok(i.clone()),
            None => Ok(self.downloads.read()?.iter().find(|i| i.files.iter().any(|j| &j.id() == id)).ok_or("No download package available")?.clone())
        }
    }

    /// Return the highest id of the download list
    pub fn get_high_id(&self) -> Result<usize> {
        // get the highest child id
        let biggest_child = self.downloads.read()?.iter().map(|pck|
                pck.files.iter()
                .map(|i| i.id()).collect::<Vec<usize>>()
            ).flat_map(|i| i.into_iter())
            .collect::<Vec<usize>>();
        let biggest_child = biggest_child.iter().max().unwrap_or(&1);

        // get the highest parent id
        let biggest_parent = self.downloads.read()?.iter()
            .map(|x| x.id())
            .collect::<Vec<usize>>();
        let biggest_parent = biggest_parent.iter().max().unwrap_or(&1);

        // return the highest number
        Ok(
            if biggest_child > biggest_parent {
                *biggest_child
            }
            else {
                *biggest_parent
            }
        )
    }

    /// Send the actual status of the file info for the given id to all
    /// connected websocket clients
    pub fn ws_send_change(&self) -> Result<()> {
        let f_info = self.get_downloads()?;
        self.sender.lock()?.send(f_info)?;
        //self.ws_send(f_info)
        Ok(())
    }

    pub fn recv_update(&self) -> Result<Vec<DownloadPackage>> {
        Ok(self.receiver.lock()?.recv()?)
    }

    fn save(&self) -> Result<()> {
        let d_list = ::serde_json::to_string_pretty(&(self.get_high_id()?, self.get_downloads()?))?;
        let mut file = File::create("./config/status.json")?;
        file.write_all(&d_list.into_bytes())?;

        Ok(())
    }

    fn load(&self) -> Result<()> {
        let file = File::open("./config/status.json")?;       
        let (id, d_list) : (usize, Vec<DownloadPackage>) = ::serde_json::from_reader(file)?;
        
        package::set_idcounter(id);

        for mut p in d_list {
            // reset speed
            p.files.iter_mut().for_each(|f| {
                f.speed = 0;
                if f.status == FileStatus::Downloading {
                    f.status = FileStatus::DownloadQueue;
                };
            });

            self.add_package(p)?;
        }

        Ok(())
    }
}

impl Default for DownloadList {
    fn default() -> Self {
        Self::new()
    }  
}