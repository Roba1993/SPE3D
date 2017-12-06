use error::*;
use std::sync::{Arc, RwLock};
use std::io::Read;
use package::{DownloadPackage, FileStatus};
use shareonline::ShareOnline;
use std::thread;
use downloader::FileWriter;


#[derive(Debug, Clone)]
pub struct DownloadManager {
    status: Arc<RwLock<DownloadManagerStatus>>,
    settings: Arc<RwLock<DownloadManagerSettings>>,
    so: Arc<RwLock<ShareOnline>>,
    links: Arc<RwLock<Vec<DownloadPackage>>>,
    downloads: Arc<RwLock<Vec<DownloadPackage>>>,
}

impl DownloadManager {
    /// Create a new Download Manager
    pub fn new<S: Into<String>>(usr: S, pwd: S) -> Result<DownloadManager> {
        Ok(DownloadManager {
            status: Arc::new(RwLock::new(DownloadManagerStatus::Running)),
            settings: Arc::new(RwLock::new(DownloadManagerSettings::new())),
            so: Arc::new(RwLock::new(ShareOnline::new(usr, pwd)?)),
            links: Arc::new(RwLock::new(vec!())),
            downloads: Arc::new(RwLock::new(Vec::new())),
        })
    }

    /// Add a new download link to the manager.
    pub fn add_link<S: Into<String>>(&mut self, url: S) -> Result<()> {
        // download the file info
        let fi = self.so.read()?.download_file_info(url.into())?;

        // create a package for the file
        let dp = DownloadPackage::new(fi.name.clone(), vec!(fi));

        // add to links
        self.links.write()?.push(dp);

        Ok(())
    }

    /// Get a copy of the link list
    pub fn get_links(&self) -> Result<Vec<DownloadPackage>> {
        match self.links.read() {
            Ok(l) => Ok(l.clone()),
            Err(_) => bail!("Can't access the donwload links")
        }
    }

    /// Start the download of an package, by the id
    pub fn start_download(&mut self, id: usize) -> Result<()> {
        // the package we want to start the downlaod for
        let pck;

        // create the lock inside the brackets
        {
            // acquire write lock
            let mut l = self.links.write()?;

            // get the position of the package
            let pos = l.iter().position(|l| l.id() == id).ok_or("The link id didn't exist")?;

            // get the package from the list
            pck = l.remove(pos);
        }

        // add the package to the download list
        self.downloads.write()?.push(pck);

        Ok(())
    }

    // start the download manager
    pub fn start(&self) -> thread::JoinHandle<()> {
        // clone the download manager to use it in the thread
        let mut dm = self.clone();

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

            // get all open download id's
            let ids = self.links.read()?.iter().map(|pck|
                    pck.files.iter()
                    .filter(|i| i.status == FileStatus::Online)
                    .map(|i| i.id()).collect::<Vec<usize>>()
                ).flat_map(|i| i.into_iter())
                .collect::<Vec<usize>>();

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

            // wait 100 ms to continue the loop
            thread::sleep(::std::time::Duration::from_millis(50));
        }
    }

    fn start_download_process(&self, d_id: &usize) -> Result<()> {
        // define the stream object this thread executes
        let mut stream;
        
        // create the stream
        {
            let mut links = self.links.write()?;

            let mut file_info = links.iter_mut()
                .flat_map(|i| i.files.iter_mut())
                .find(|ref i| d_id == &i.id()).ok_or("The can't be found")?;

            file_info.status = FileStatus::Downloading;

            stream = self.so.write()?.download_file(file_info)?;
        }
        
        // start the download
        let hash = stream.write_to_file("./out/new.txt")?;
        println!("HASH FROM DLOAD: {}", hash);
        
        // check the download and update the status
        {
            // writeblock the links
            let mut links = self.links.write()?;

            // get the right file info
            let mut file_info = links.iter_mut()
                .flat_map(|i| i.files.iter_mut())
                .find(|ref i| d_id == &i.id()).ok_or("The can't be found")?;

            // check if the hash matched
            if hash == file_info.hash.md5().ok_or("No MD5 hash available")? {
                file_info.status = FileStatus::Downloaded;
            }
            else {
                file_info.status = FileStatus::WrongHash;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DownloadManagerStatus {
    Running,
    EndAfterDownloads,
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