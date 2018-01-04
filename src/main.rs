#![feature(match_default_bindings)]
#![feature(const_atomic_usize_new)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate error_chain;
#[macro_use]extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate rocket;
extern crate rocket_contrib;
extern crate rocket_cors;
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
use rocket::response::NamedFile;
use rocket::State;
use rocket_contrib::Json;
use std::path::Path;
use manager::DownloadManager;
use package::DownloadPackage;

fn main() {
    let mut dm = DownloadManager::new("96999125025", "wDkEIBdaQ").unwrap();
    dm.add_link("http://www.share-online.biz/dl/6HE8ZA0PXQM8").unwrap();
    dm.start();

    rocket::ignite()
        .manage(dm)
        .attach(rocket_cors::Cors::default())
        .mount("/", routes![api_start_download, api_downloads, index, files])
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