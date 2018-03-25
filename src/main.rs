#![feature(match_default_bindings)]
#![feature(const_atomic_usize_new)]
#![feature(plugin)]
#![plugin(rocket_codegen)]
#![plugin(clippy)]

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
use dlc_decrypter::DlcDecoder;
use config::Config;
use std::convert::From;
use error_chain::ChainedError;

fn main() {
    // load the config file
    let config = Config::new();

    // create the download manager
    let dm = DownloadManager::new(config.clone()).unwrap();
    dm.start();

    // start the websocket server and add it to the download manager
    dm.set_ws_sender(websocket::start_ws(&config)).unwrap();
    
    // start the rocket webserver
    rocket::custom(config.into(), true)
        .manage(dm)
        .attach(rocket_cors::Cors::default())
        .mount("/", routes![api_test, api_start_download, api_downloads, api_add_links, api_add_dlc, index, files])
        .launch();
}


#[get("/")]
fn index() -> ::std::io::Result<NamedFile> {
    NamedFile::open("www/index.html")
}

#[get("/<file>")]
fn files(file: String) -> Option<NamedFile> {
    NamedFile::open(Path::new("www/").join(file)).ok()
}

#[get("/api/test")]
fn api_test() -> String {
    "Success".to_string()
}

#[get("/api/downloads")]
#[allow(needless_pass_by_value)]
fn api_downloads(dm: State<DownloadManager>) -> Result<Json<Vec<DownloadPackage>>> {
    Ok(Json(dm.get_downloads()?))
}

#[post("/api/start-download/<id>")]
#[allow(needless_pass_by_value)]
fn api_start_download(dm: State<DownloadManager>, id: usize) -> Result<()> {
    dm.start_download(id)
}

#[post("/api/add-links", data = "<json>")]
#[allow(needless_pass_by_value)]
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
#[allow(needless_pass_by_value)]
fn api_add_dlc(dm: State<DownloadManager>, data: String) -> Result<()> {
    println!("DLC UPLOAD => START");
    let t = match tmp(&dm, &data) {
        Ok(_) => {println!("Added DLC"); Ok(())},
        Err(e) => {println!("{}", e.display_chain().to_string()); Err(e)}
    };
    println!("DLC UPLOAD => END");
    t
}

fn tmp(dm: &State<DownloadManager>, data: &str) -> Result<()> {
    // extract the dlc package
    println!("DLC UPLOAD => CREATE DLC");
    let dlc = DlcDecoder::new();
    println!("DLC UPLOAD => CREATE PCK");
    let pck = dlc.from_data(data.as_bytes())?;

    // add it to the manager
    println!("DLC UPLOAD => ADD PCK");
    dm.add_package(pck)?;

    Ok(())
}