//! Holding all data models which are shared across the complete crate.

use error::*;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use dlc_decrypter::DlcPackage;
use std::sync::{Arc, RwLock, Mutex};
use std::sync::mpsc::{Sender, Receiver, channel};
use std::fs::File;
use std::io::prelude::*;



// counter for the unique id's of the download packages
static IDCOUNTER: AtomicUsize = AtomicUsize::new(1);

/// Set the internal counter to a new number, to allow 
/// the correct creation of new id's for the data models
pub fn set_idcounter(id: usize) {
    if id > IDCOUNTER.load(Ordering::Relaxed) {
        IDCOUNTER.store(id, Ordering::Relaxed);
        IDCOUNTER.fetch_add(1, Ordering::SeqCst);
    }
}



/// Data model for a single download container, which can hold
/// multiple download links. 
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DownloadPackage {
    id: usize,
    pub name: String,
    pub files: Vec<DownloadFile>,
}

impl DownloadPackage {
    pub fn new<S: Into<String>>(name: S, files: Vec<DownloadFile>) -> DownloadPackage {
        DownloadPackage {
            id: IDCOUNTER.fetch_add(1, Ordering::SeqCst),
            name: name.into(),
            files
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

impl From<DlcPackage> for DownloadPackage {
    fn from(dlc: DlcPackage) -> Self {
        let files = dlc.files.into_iter().map(|i| {
            let mut f = DownloadFile::new(); 
            f.url = i.url; 
            f.name = i.name; 
            f.size = i.size.parse().unwrap_or(0);
            f
        }).collect();

        DownloadPackage::new(dlc.name, files)
    }
}


/// Data model for a single link/file which can be downloaded.
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct DownloadFile {
    id: usize,
    pub status: FileStatus,
    pub host: FileHoster,
    pub name: String,
    pub url: String,
    pub size: usize,
    pub downloaded: usize,
    pub speed: usize,
    pub hash: FileHash,
    pub infos: HashMap<String, String>
}

impl DownloadFile {
    pub fn new() -> DownloadFile {
        DownloadFile {
            id: IDCOUNTER.fetch_add(1, Ordering::SeqCst),
            status: FileStatus::Unknown,
            host: FileHoster::Unknown,
            name: "".to_string(),
            url: "".to_string(),
            size: 0,
            downloaded: 0,
            speed: 0,
            hash: FileHash::None,
            infos: HashMap::new(),
        }
    }

    pub fn default() -> Self {
        Self::new()
    }

    pub fn id(&self) -> usize {
        self.id
    }
}



/// Enumeration of the supported hoster, where spe3d can load from.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum FileHoster {
    Unknown,
    ShareOnline,
}

impl Default for FileHoster {
    fn default() -> Self {
        FileHoster::Unknown
    }
}



/// List of the different status a single file can have.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
pub enum FileStatus {
    Unknown,
    Offline,
    Online,
    DownloadQueue,
    Downloading,
    Downloaded,
    WrongHash,
}

impl Default for FileStatus {
    fn default() -> Self {
        FileStatus::Unknown
    }
}



/// Support for No hash or a Md5 hash
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum FileHash {
    None,
    Md5(String),
}

impl FileHash {
    pub fn md5(&self) -> Option<String> {
        match self {
            FileHash::Md5(h) => Some(h.clone()),
            _ => None
        }
    }
}

impl Default for FileHash {
    fn default() -> Self {
        FileHash::None
    }
}



/// A temporary list of all `DownloadPackage`s
/// This is the format mostly returned by the `DownloadManager`
type DownloadList = Vec<DownloadPackage>;



/// Save shareable kist off all the download packages and file informations.
/// This structure is mostly used internally by the `DownloadManager`.
///
/// The DownloadList which is mostly used by the crate user, is a snapshot of this
/// list at a certain time.
#[derive(Clone)]
pub struct SmartDownloadList {
    downloads: Arc<RwLock<DownloadList>>,
    sender: Arc<Mutex<Sender<DownloadList>>>,
    receiver: Arc<Mutex<Receiver<DownloadList>>>,
}

impl SmartDownloadList {
    /// Create a new Download Manager
    pub fn new() -> SmartDownloadList {
        let (sender, receiver) = channel();

        // create the download list
        let d_list = SmartDownloadList {
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
    pub fn get_downloads(&self) -> Result<DownloadList> {
        Ok(self.downloads.read()?.clone())
    }

    /// Gives a list of the files with the status back
    pub fn files_status(&self, status: FileStatus) -> Result<DownloadList> {
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
    pub fn get_file(&self, id: &usize) -> Result<DownloadFile> {
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

    pub fn recv_update(&self) -> Result<DownloadList> {
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
        let (id, d_list) : (usize, DownloadList) = ::serde_json::from_reader(file)?;
        
        ::models::set_idcounter(id);

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

impl Default for SmartDownloadList {
    fn default() -> Self {
        Self::new()
    }  
}