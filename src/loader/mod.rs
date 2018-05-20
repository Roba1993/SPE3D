//! The loader handles the different loader implementations and manage
//! them for the `DownloadManager`.

pub mod so;

use error::*;
use models::{DownloadFile, FileStatus, SmartDownloadList};
use self::so::ShareOnline;
use config::Config;
use std::thread;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::sync::{Arc, Mutex};
use std::io::Read;
use md5::{Md5, Digest};
use std::fs::File;
use std::io::Write;
use std::time::{Duration, Instant};


/// This `Loader` defines which funtionalities are used to download a file from a source
pub trait Loader {
    /// Check a url if it can be loaded with the laoder. Retuns an error if not
    fn check(&self, url: &str) -> Result<Option<DownloadFile>>;

    /// Download a file, with this laoder
    fn download(&self, file: &DownloadFile) -> Result<::reqwest::Response>;

    /// Prove that the downloaded file is correct
    fn prove(&self, file: &DownloadFile, path: &str) -> Result<bool>;
}






/// The `Downloader` manages the actual downloads of files throgh different loader implementations.
#[derive(Clone)]
pub struct Downloader {
    config: Arc<Config>,
    d_list: Arc<SmartDownloadList>,
    d_updater: Arc<DownloadUpdater>,
    loader: Arc<Vec<Box<Loader+Sync+Send>>>
}

impl Downloader {
    /// Create a new Downloader
    pub fn new(config: Config, d_list: SmartDownloadList) -> Downloader {
        let loader = Arc::new(vec!(
            Box::new(ShareOnline::new(config.clone())) as Box<Loader+Sync+Send>,
        ));

         Downloader {
            config: Arc::new(config),
            d_list: Arc::new(d_list.clone()),
            d_updater: Arc::new(DownloadUpdater::new(d_list)),
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
            if let Ok(Some(f)) = l.check(&link) {
                return Ok(f);
            }
        }

        Err(Error::from("Can't identify file info"))
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
        let hash = stream.write_to_file(path.clone(), &f_info.id(), self.d_updater.get_sender()?)?;
        println!("HASH FROM DLOAD: {}", hash);

        // set the downloaded attribute to the size, because all is downloaded and set speed to 0
        self.d_list.add_downloaded(f_info.id(), 0)?;
        self.d_list.set_downloaded(f_info.id(), f_info.size)?;

        // check if the download can be proven
        if self.loader.iter().any(|l| l.prove(&f_info, &path).unwrap_or(false)) {
            self.d_list.set_status(id, &FileStatus::Downloaded)?;
        }
        else {
            self.d_list.set_status(id, &FileStatus::WrongHash)?;
        }

        Ok(())
    }
}






/// The download updater receives the actual download speed and volume for each file
/// and reports this back to the `DownloadList`.
#[derive(Clone)]
pub struct DownloadUpdater {
    sender: Arc<Mutex<Sender<(usize, usize)>>>,
    d_list: SmartDownloadList,
}

impl DownloadUpdater {
    /// Create a new download updater
    pub fn new(d_list: SmartDownloadList) -> DownloadUpdater {
        let (sender, receiver) = channel();

        let updater = DownloadUpdater {
            sender: Arc::new(Mutex::new(sender)),
            d_list,
        };

        updater.run(receiver);

        updater
    }

    /// Get a sender to send messages/updates to the downlaod updater.
    /// The Sender needs the following data (id, data last second)
    pub fn get_sender(&self) -> Result<Sender<(usize, usize)>> {
        Ok(self.sender.lock()?.clone())
    }

    /// Start the download updater (spawns a thread)
    fn run(&self, receiver: Receiver<(usize, usize)>) {
        let this = self.clone();

        // new thread for the download
        thread::spawn(move || {
            loop {
                match this.handle_update(&receiver) {
                    Ok(_) => {},
                    Err(e) => {println!("{}", e)}
                }
            }
        });
    }

    /// Internal function to handle the incomingmessages
    fn handle_update(&self, receiver: &Receiver<(usize, usize)>) -> Result<()> {
        let (id, size) = receiver.recv()?;
        self.d_list.add_downloaded(id, size)
    }
}




/// Trait to write a stream of data to a file.
pub trait FileWriter : Read {
    /// Function to write a stream of data, to a file
    /// based on the std::io::Read trait. This functions
    /// returns as result the hash of the written file.
    fn write_to_file<S: Into<String>>(&mut self, file: S, id: &usize, sender: Sender<(usize, usize)>) -> Result<String> {
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
                sender.send((*id, speed))?;
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