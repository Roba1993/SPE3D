//! Holding all data models which are shared across the complete crate.

use error::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use dlc_decrypter::DlcPackage;
use std::sync::{Arc, RwLock, Mutex};
use std::sync::mpsc::{Sender, Receiver};
use std::fs::File;
use std::io::prelude::*;
use bus::{MessageBus, Message};
use std::thread;


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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DownloadFile {
    id: usize,
    pub status: FileStatus,
    pub hoster: FileHoster,
    pub name: String,
    pub url: String,
    pub size: usize,
    pub downloaded: usize,
    pub speed: usize,
    pub hash: FileHash,
    pub file_id: String,
}

impl DownloadFile {
    pub fn new() -> DownloadFile {
        DownloadFile {
            id: IDCOUNTER.fetch_add(1, Ordering::SeqCst),
            status: FileStatus::Unknown,
            hoster: FileHoster::Unknown,
            name: "".to_string(),
            url: "".to_string(),
            size: 0,
            downloaded: 0,
            speed: 0,
            hash: FileHash::None,
            file_id: "".to_string(),
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



/// CaptchaResult structure to receive from captcha
/// solving plugin
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CaptchaResult {
    pub id: usize,
    pub file_id: String,
    pub hoster: String,
    pub url: String
}





/// A temporary list of all `DownloadPackage`s
/// This is the format mostly returned by the `DownloadManager`
pub type DownloadList = Vec<DownloadPackage>;



/// Save shareable kist off all the download packages and file informations.
/// This structure is mostly used internally by the `DownloadManager`.
///
/// The `DownloadList` which is mostly used by the crate user, is a snapshot of this
/// list at a certain time.
#[derive(Clone)]
pub struct SmartDownloadList {
    downloads: Arc<RwLock<DownloadList>>,
    sender: Arc<Mutex<Sender<Message>>>,
    receiver: Arc<Mutex<Receiver<Message>>>,
}

impl SmartDownloadList {
    /// Create a new Download Manager
    pub fn new(bus: &MessageBus) -> Result<SmartDownloadList> {
        let (sender, receiver) = bus.channel()?;

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

        // a clone from the msg bus for the internal handler
        let d_list_internal = d_list.clone();
        // new thread for the bus handler
        thread::spawn(move || loop {
            match d_list_internal.handle_msg() {
                Ok(_) => {}
                Err(e) => println!("{}", e),
            }
        });

        // return the download list
        Ok(d_list)
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

        self.publish_update()?;
        self.save()
    }

    /// Add the size of data which is additionally downloaded.
    /// This function does not trigger an message to the bus.
    /// This function is autmatically triggered by the corresponding
    /// bus message
    pub fn add_downloaded(&self, id: usize, size: usize) -> Result<()> {
        self.downloads.write()?.iter_mut().for_each(|pck| 
            if let Some(i) = pck.files.iter_mut().find(|f| f.id() == id) { 
                i.downloaded += size;
                i.speed = size;
            }
        );

        Ok(())
    }

    pub fn set_downloaded(&self, id: usize, size: usize) -> Result<()> {
        self.downloads.write()?.iter_mut().for_each(|pck| 
            if let Some(i) = pck.files.iter_mut().find(|f| f.id() == id) { 
                i.downloaded = size;
            }
        );

        self.publish_update()?;
        Ok(())
    }

    /// Add a new package to the download list
    pub fn add_package(&self, package: DownloadPackage) -> Result<()> {
        self.downloads.write()?.push(package);
        self.publish_update()?;
        self.save()
    }

    /// Add a new package to the download list
    pub fn remove(&self, id: usize) -> Result<()> {
        // delete a children
        self.downloads.write()?.iter_mut().for_each(|p| p.files.retain(|i| i.id() != id));
        // delete a container also all empty containers
        self.downloads.write()?.retain(|i| i.id() != id && !i.files.is_empty() );

        self.publish_update()?;
        self.save()
    }

    /// Get a copy of the download list
    pub fn get_downloads(&self) -> Result<DownloadList> {
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

    /// Gives a list of the files with the status back
    pub fn files_status_hoster(&self, status: FileStatus, hoster: FileHoster) -> Result<Vec<usize>> {
        // get all download id's in queue to start
        let ids = self.downloads.read()?.iter().map(|pck|
                pck.files.iter()
                .filter(|i| i.status == status && i.hoster == hoster)
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

    /// Sends the actual status of the `SmartDownloaList` as a `DownloadList`
    /// into the update channel.
    ///
    /// This update can be automatically received by the function `recv_update()`
    pub fn publish_update(&self) -> Result<()> {
        // get the download list and send it into the channel
        self.sender.lock()?.send(Message::DownloadList(self.get_downloads()?))?;
        Ok(())
    }

    /// handles incoming update speed messages
    fn handle_msg(&self) -> Result<()> {
        if let Message::DownloadSpeed((id, size)) = self.receiver.lock()?.recv()? {
            self.add_downloaded(id, size)?;
        }

        Ok(())
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