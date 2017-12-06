use error::*;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::io::Read;

// counter for the unique id's of the download packages
static IDCOUNTER: AtomicUsize = AtomicUsize::new(1);

#[derive(Debug, Clone)]
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
            files: files
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

#[derive(Debug, Clone)]
pub struct DownloadFile {
    id: usize,
    pub status: FileStatus,
    pub host: FileHoster,
    pub name: String,
    pub url: String,
    pub size: u64,
    pub hash: FileHash,
    //pub download: Option<Read>,
    pub infos: HashMap<&'static str, String>
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
            hash: FileHash::None,
            infos: HashMap::new(),
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FileHoster {
    Unknown,
    ShareOnline,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FileStatus {
    Unknown,
    Online,
    Offline,
    Downloading,
    Downloaded,
    WrongHash,
}

#[derive(Debug, Clone, PartialEq)]
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