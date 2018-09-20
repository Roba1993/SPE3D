//! The loader handles the different loader implementations and manage
//! them for the `DownloadManager`.

pub mod so;

use error::*;
use models::{DownloadFile, FileStatus, SmartDownloadList};
use self::so::ShareOnline;
use config::Config;
use std::thread;
use std::sync::{Arc};
use std::io::Read;
use md5::{Md5, Digest};
use std::fs::File;
use std::io::Write;
use std::time::{Duration, Instant};
use bus::{MessageBus, Message};


/// This `Loader` defines which funtionalities are used to download a file from a source
pub trait Loader {
    /// This function updates an Share-Online account with the actual status
    fn update_account(&self, account: &mut ::config::ConfigAccount) -> Result<()> ;

    /// Check the download url and return the file info
    fn check_url(&self, url: &str) -> Result<Option<DownloadFile>>;

    /// Download a file, with this laoder
    fn download(&self, file: &DownloadFile) -> Result<::reqwest::Response>;

    /// Prove that the downloaded file is correct
    fn prove_download(&self, file: &DownloadFile, path: &str) -> Result<bool>;
}






/// The `Downloader` manages the actual downloads of files throgh different loader implementations.
#[derive(Clone)]
pub struct Downloader {
    config: Config,
    d_list: SmartDownloadList,
    bus: MessageBus,
    loader: Arc<Vec<Box<Loader+Sync+Send>>>
}

impl Downloader {
    /// Create a new Downloader
    pub fn new(config: Config, d_list: SmartDownloadList, bus: MessageBus) -> Downloader {
        let loader = Arc::new(vec!(
            Box::new(ShareOnline::new(config.clone())) as Box<Loader+Sync+Send>,
        ));

         Downloader {
            config,
            d_list,
            bus,
            loader
        }
    }

    /// Download a file
    pub fn download(&self, id: usize) {
        let this = self.clone();

        // new thread for the download
        thread::spawn(move || {
            this.internal_download(id).unwrap();
        });
    }

    /// Check the status of a file
    pub fn check<S: Into<String>>(&self, link: S) -> Result<DownloadFile> {
        let link = link.into();

        // loop over all the loader
        for l in self.loader.iter() {
            // return the first valid file check info
            if let Ok(Some(f)) = l.check_url(&link) {
                return Ok(f);
            }
        }

        Err(Error::from("Can't identify file info"))
    }

    /// Update and complete the account data
    pub fn update_account(&self, account: &mut ::config::ConfigAccount) -> Result<()> {
        // loop over all the loader
        for l in self.loader.iter() {
            // each loader can try to update the account
            if let Ok(_) = l.update_account(account) {
                // when a loader was responsible set the updated time
                account.checked = ::std::time::SystemTime::now();
            }
        }

        Ok(())
    }

    /*************************** Private Functions ************************/
    /// The detail downloading process
    fn internal_download(&self, id: usize) -> Result<()> {
        // set the status to downloading
        self.d_list.set_status(id, &FileStatus::Downloading)?;
        // get the file info
        let f_info = self.d_list.get_file(&id)?;
        let pck = self.d_list.get_package(&id)?;
        let path = format!("./out/{}/{}", pck.name, f_info.name);

        let mut stream = None;

        for d in self.loader.iter() {
            if let Ok(f) = d.download(&f_info) {
                stream = Some(f);
                break;
            }
        }

        let mut stream = match stream {
            Some(s) => s,
            None => {bail!("No stream available");}
        };

        // set the download status to zero
        self.d_list.set_downloaded(f_info.id(), 0)?;

        ::std::fs::create_dir_all(format!("./out/{}", pck.name))?;
        let hash = stream.write_to_file(path.clone(), f_info.id(), &self.bus)?;
        println!("HASH FROM DLOAD: {}", hash);

        // set the downloaded attribute to the size, because all is downloaded and set speed to 0
        self.d_list.add_downloaded(f_info.id(), 0)?;
        self.d_list.set_downloaded(f_info.id(), f_info.size)?;

        // check if the download can be proven
        if self.loader.iter().any(|l| l.prove_download(&f_info, &path).unwrap_or(false)) {
            self.d_list.set_status(id, &FileStatus::Downloaded)?;
        }
        else {
            self.d_list.set_status(id, &FileStatus::WrongHash)?;
        }

        Ok(())
    }
}


/// Trait to write a stream of data to a file.
pub trait FileWriter : Read {
    /// Function to write a stream of data, to a file
    /// based on the std::io::Read trait. This functions
    /// returns as result the hash of the written file.
    fn write_to_file<S: Into<String>>(&mut self, file: S, id: usize, bus: &MessageBus) -> Result<String> {
        // get the sender
        let sender = bus.get_sender()?;

        // define the buffer
        let mut buffer = [0u8; 4096];
        let mut start = Instant::now();

        // define the hasher
        let mut hasher = Md5::new();

        // Create the output file
        let mut file = File::create(file.into())?;
        let mut speed = 0;

        // print out the values
        loop {
            // read the data from the stream
            let len = self.read(&mut buffer)?;
            speed += len;

            // break if no data is available anymore
            if len == 0 {
                break;
            }

            // sent the data to the file and hasher
            hasher.input(&buffer[0..len]);
            file.write_all(&buffer[0..len])?;

            // update the status
            if start.elapsed() > Duration::from_secs(1) {
                sender.send(Message::DownloadSpeed((id, speed)))?;
                speed = 0;
                start = Instant::now();
            }
        }

        // return the hash as a string
        Ok(format!("{:x}", hasher.result()))
    }
}

// implement the Download Reader for the reqwest response
impl FileWriter for ::reqwest::Response{}