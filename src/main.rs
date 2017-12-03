#[macro_use] extern crate error_chain;
extern crate dlc_decrypter;
extern crate reqwest;
extern crate regex;

pub mod error;
pub mod dlc;
pub mod shareonline;

use error::*;

use std::io::Read;
use reqwest::Client;
use regex::Regex;

fn main() {
    for arg in std::env::args().skip(1){
        let dlc = dlc::read_dlc(&arg).unwrap();
        let file = & dlc.files.get(1).unwrap().url;
        let file = "http://www.share-online.biz/dl/6HE8ZA0PXQM8";
        println!("file: {}", &file);


        let so = shareonline::ShareOnline::new("96999125025", "wDkEIBdaQ").unwrap();
        let mut dl = so.download(file.clone()).unwrap();

        let mut content = String::new();
        dl.read_to_string(&mut content).unwrap();
        println!("Content: {}", content);
    }
}