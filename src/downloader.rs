use error::*;
use package::{DownloadFile, FileHoster, FileStatus};
use shareonline::ShareOnline;
use std::io::Read;
use manager::DownloadList;
use config::Config;
use std::thread;
use writer::FileWriter;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Downloader {
    config: Config,
    d_list: DownloadList,
    so_loader: Option<ShareOnline>,
    d_updater: DownloadUpdater,
}

impl Downloader {
    pub fn new(config: Config, d_list: DownloadList) -> Downloader {
         Downloader {
            config: config.clone(),
            d_list: d_list.clone(),
            //so_loader: ShareOnline::new(config.get().share_online.unwrap().username, config.get().share_online.unwrap().password).ok(),
            so_loader: None,
            d_updater: DownloadUpdater::new(d_list),
        }
    }

    pub fn download(&self, id: usize) {
        let this = self.clone();

        // new thread for the download
        thread::spawn(move || {
            this.internal_download(id).unwrap();
        });
    }

    pub fn check<S: Into<String>>(&self, link: S) -> Result<DownloadFile> {
        let link = link.into();

        // check Share-Online
        let file_info = match self.so_loader.as_ref() {
            Some(d) => d.check(link)?,
            None => None
        };
        if file_info.is_some() { return file_info.ok_or(Error::from("File lost")) };

        Err(Error::from("Can't identify file info"))
    }

    /*************************** Private Functions ************************/

    fn internal_download(&self, id: usize) -> Result<()> {
        // set the status to downloading
        self.d_list.set_status(id.clone(), FileStatus::Downloading)?;
        // get the file info
        let f_info = self.d_list.get_file(&id)?;
        let pck = self.d_list.get_package(&id)?;

        // get the download stream
        let mut stream = match f_info.host {
            FileHoster::ShareOnline => self.so_loader.as_ref().ok_or("Share-Online downloader not available")?.download_file(&f_info),
            _ => Err(Error::from("Hoster not supported"))
        }?;

        ::std::fs::create_dir_all(format!("./out/{}", pck.name))?;
        let hash = stream.write_to_file(format!("./out/{}/{}", pck.name, f_info.name), &f_info.id(), self.d_updater.get_sender()?)?;
        println!("HASH FROM DLOAD: {}", hash);

        // check if the hash matched
        if hash == f_info.hash.md5().ok_or("No MD5 hash available")? {
            self.d_list.set_status(id.clone(), FileStatus::Downloaded);
        }
        else {
            self.d_list.set_status(id.clone(), FileStatus::WrongHash);
        }

        Ok(())
    }
}

#[derive(Clone)]
pub struct DownloadUpdater {
    sender: Arc<Mutex<Sender<(usize, usize)>>>,
    d_list: DownloadList,
}

impl DownloadUpdater {
    pub fn new(d_list: DownloadList) -> DownloadUpdater {
        let (sender, receiver) = channel();

        let updater = DownloadUpdater {
            sender: Arc::new(Mutex::new(sender)),
            d_list: d_list,
        };

        updater.run(receiver);

        updater
    }

    pub fn get_sender(&self) -> Result<Sender<(usize, usize)>> {
        Ok(self.sender.lock()?.clone())
    }

    fn run(&self, receiver: Receiver<(usize, usize)>) {
        let this = self.clone();

        // new thread for the download
        thread::spawn(move || {
            loop {
                this.handle_update(&receiver);
            }
        });
    }

    fn handle_update(&self, receiver: &Receiver<(usize, usize)>) -> Result<()> {
        let (id, size) = receiver.recv()?;
        self.d_list.set_downloaded(id, size)
    }
}