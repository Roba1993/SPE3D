#![feature(match_default_bindings)]
#![feature(const_atomic_usize_new)]

#[macro_use] extern crate error_chain;
extern crate dlc_decrypter;
extern crate reqwest;
extern crate regex;
extern crate md5;

pub mod error;
pub mod package;
pub mod shareonline;
pub mod manager;
pub mod downloader;

use error::*;

fn main() {
    let mut dm = manager::DownloadManager::new("96999125025", "wDkEIBdaQ").unwrap();
    dm.add_link("http://www.share-online.biz/dl/6HE8ZA0PXQM8").unwrap();
    dm.start().join().unwrap();
}