#![feature(match_default_bindings)]
#![feature(const_atomic_usize_new)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate error_chain;
#[macro_use]extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate toml;
extern crate rocket;
extern crate rocket_contrib;
extern crate rocket_cors;
extern crate ws;
extern crate dlc_decrypter;
extern crate reqwest;
extern crate regex;
extern crate md5;

pub mod error;
pub mod config;
pub mod package;
pub mod shareonline;
pub mod manager;
pub mod writer;
pub mod downloader;
pub mod websocket;

use error::*;
use rocket::response::NamedFile;
use rocket::State;
use rocket_contrib::Json;
use std::path::Path;
use manager::DownloadManager;
use package::DownloadPackage;
use regex::Regex;
use dlc_decrypter::DlcDecoder;
use config::Config;

fn main() {
    // load the config file
    let config = Config::new();

    // create the download manager
    let mut dm = DownloadManager::new(config).unwrap();
    dm.start();

    // start the websocket server and add it to the download manager
    dm.set_ws_sender(websocket::start_ws()).unwrap();

    // add a link
    dm.add_links("MOD 1080 Example", vec!("http://www.share-online.biz/dl/6HE8ZA0PXQM8".to_string())).unwrap();
    
    // start the rocket webserver
    rocket::ignite()
        .manage(dm)
        .attach(rocket_cors::Cors::default())
        .mount("/", routes![api_start_download, api_downloads, api_add_links, api_add_dlc, index, files])
        .launch();
}

#[get("/")]
fn index() -> ::std::io::Result<NamedFile> {
    NamedFile::open("www/build/index.html")
}

#[get("/<file>")]
fn files(file: String) -> Option<NamedFile> {
    NamedFile::open(Path::new("www/build/").join(file)).ok()
}

#[get("/api/downloads")]
fn api_downloads(dm: State<DownloadManager>) -> Result<Json<Vec<DownloadPackage>>> {
    Ok(Json(dm.get_downloads()?))
}

#[post("/api/start-download/<id>")]
fn api_start_download(dm: State<DownloadManager>, id: usize) -> Result<()> {
    dm.start_download(id)
}

#[post("/api/add-links", data = "<json>")]
fn api_add_links(dm: State<DownloadManager>, json: Json<serde_json::Value>) -> Result<()> {
    // add the links as a package
    dm.add_links(
        // get the name
        json["name"].as_str().ok_or("Package name is not provided")?,
        // get the links
        json["links"].as_array().ok_or("Package links are not provided")?.iter().map(|u| u.as_str()).filter(|u| u.is_some()).map(|u| u.unwrap().to_string()).collect()
    )
}

#[post("/api/add-dlc", data = "<data>")]
fn api_add_dlc(dm: State<DownloadManager>, data: String) -> Result<()> {
    // find the indentifier
    let re = Regex::new(r"-*\d*")?;
    let ident = re.find(&data).ok_or("No file data")?.as_str();

    // find the file start
    let re = Regex::new(r"\r\n\r\n")?;
    let start = re.find(&data).ok_or("No start")?.end();

    // find the file end
    let re = Regex::new(&format!("(\r\n{})",&ident))?;
    let end = re.find(&data).ok_or("No end")?.start();

    // extract the dlc package
    let dlc = DlcDecoder::new();
    let pck = dlc.from_data(&data[start..end].as_bytes())?;

    // add it to the manager
    dm.add_package(pck)
}