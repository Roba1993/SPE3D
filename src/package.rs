use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use dlc_decrypter::DlcPackage;

// counter for the unique id's of the download packages
static IDCOUNTER: AtomicUsize = AtomicUsize::new(1);

pub fn set_idcounter(id: usize) {
    if id > IDCOUNTER.load(Ordering::Relaxed) {
        IDCOUNTER.store(id, Ordering::Relaxed);
        IDCOUNTER.fetch_add(1, Ordering::SeqCst);
    }
}

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