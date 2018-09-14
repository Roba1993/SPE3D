#[macro_use]
extern crate error_chain;
extern crate warp;
extern crate bytes;
extern crate serde;
extern crate serde_json;
extern crate spe3d;
extern crate ws;

pub mod error;
pub mod websocket;

use spe3d::config::Config;
use spe3d::DownloadManager;

use bytes::Buf;
use warp::Filter;
use std::io::Read;

fn main() {
    // load the config file
    let config = Config::new();

    // create the download manager
    let dm = DownloadManager::new(config.clone().into()).unwrap();
    dm.start();

    // start the websocket server and add it to the download manager
    websocket::start_ws(&config, &dm);

    // warp webserver code
    let dm = warp::any().map(move || dm.clone());

    let api = warp::path("api");

    let get_files = warp::fs::dir("./www/");

    let get_index = warp::get2()
        .and(warp::index())
        .and(warp::fs::file("./www/index.html"));

    let get_test = warp::get2()
        .and(api.and(warp::path("test")).and(warp::path::index()))
        .and_then(get_test)
        .with(warp::reply::with::header(
            "Access-Control-Allow-Origin",
            "*",
        ));

    let get_downloads = warp::get2()
        .and(api.and(warp::path("downloads")).and(warp::path::index()))
        .and(dm.clone())
        .and_then(get_downloads)
        .with(warp::reply::with::header(
            "Access-Control-Allow-Origin",
            "*",
        ));

    let post_start_download = warp::post2()
        .and(
            api.and(warp::path("start-download"))
                .and(warp::path::param::<usize>())
                .and(warp::path::index()),
        )
        .and(dm.clone())
        .and_then(post_start_download)
        .with(warp::reply::with::header(
            "Access-Control-Allow-Origin",
            "*",
        ));

    let post_add_links = warp::post2()
        .and(api.and(warp::path("add-links")).and(warp::path::index()))
        .and(dm.clone())
        .and(warp::body::json())
        .and_then(post_add_links)
        .with(warp::reply::with::header(
            "Access-Control-Allow-Origin",
            "*",
        ));

    let post_delete_link = warp::post2()
        .and(
            api.and(warp::path("delete-link"))
                .and(warp::path::param::<usize>())
                .and(warp::path::index()),
        )
        .and(dm.clone())
        .and_then(post_delete_link)
        .with(warp::reply::with::header(
            "Access-Control-Allow-Origin",
            "*",
        ));

    let post_add_dlc = warp::post2()
        .and(api.and(warp::path("add-dlc")).and(warp::path::index()))
        .and(dm.clone())
        .and(warp::body::concat())
        .and_then(post_add_dlc)
        .with(warp::reply::with::header(
            "Access-Control-Allow-Origin",
            "*",
        ));

    let get_config = warp::get2()
        .and(api.and(warp::path("config")).and(warp::path::index()))
        .and(dm.clone())
        .and_then(get_config)
        .with(warp::reply::with::header(
            "Access-Control-Allow-Origin",
            "*",
        ));

    let post_config_server = warp::post2()
        .and(
            api.and(warp::path("config"))
                .and(warp::path("server"))
                .and(warp::path::index()),
        )
        .and(dm.clone())
        .and(warp::body::json())
        .and_then(post_config_server)
        .with(warp::reply::with::header(
            "Access-Control-Allow-Origin",
            "*",
        ));

    let post_config_account = warp::post2()
        .and(
            api.and(warp::path("config"))
                .and(warp::path("account"))
                .and(warp::path::index()),
        )
        .and(dm.clone())
        .and(warp::body::json())
        .and_then(post_config_account)
        .with(warp::reply::with::header(
            "Access-Control-Allow-Origin",
            "*",
        ));

    let delete_config_account = warp::delete2()
        .and(
            api.and(warp::path("config"))
                .and(warp::path("account"))
                .and(warp::path::param::<usize>())
                .and(warp::path::index()),
        )
        .and(dm.clone())
        .and_then(delete_config_account)
        .with(warp::reply::with::header(
            "Access-Control-Allow-Origin",
            "*",
        ));

    let routes = get_index
        .or(get_files)
        .or(get_test)
        .or(get_downloads)
        .or(post_start_download)
        .or(post_add_links)
        .or(post_delete_link)
        .or(post_add_dlc)
        .or(get_config)
        .or(post_config_server)
        .or(post_config_account)
        .or(delete_config_account);

    warp::serve(routes).run(([127, 0, 0, 1], 8000));
}

fn get_test() -> Result<impl warp::Reply, warp::Rejection> {
    Ok("Success")
}

fn get_downloads(dm: DownloadManager) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&dm.get_downloads().unwrap()))
}

fn post_start_download(
    id: usize,
    dm: DownloadManager,
) -> Result<impl warp::Reply, warp::Rejection> {
    dm.start_download(id).unwrap();
    Ok(warp::reply())
}

fn post_add_links(
    dm: DownloadManager,
    json: serde_json::Value,
) -> Result<impl warp::Reply, warp::Rejection> {
    // add the links as a package
    dm.add_links(
        // get the name
        json["name"]
            .as_str()
            .ok_or("Package name is not provided")
            .unwrap(),
        // get the links
        json["links"]
            .as_array()
            .ok_or("Package links are not provided")
            .unwrap()
            .iter()
            .map(|u| u.as_str())
            .filter(|u| u.is_some())
            .map(|u| u.unwrap().to_string())
            .collect(),
    ).unwrap();
    Ok("")
}

fn post_delete_link(id: usize, dm: DownloadManager) -> Result<impl warp::Reply, warp::Rejection> {
    // remove the container or link
    dm.remove(id).unwrap();
    Ok("")
}

fn post_add_dlc(
    dm: DownloadManager,
    dlc: warp::filters::body::FullBody,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut s = String::new();
    dlc.reader().read_to_string(&mut s).unwrap();

    dm.add_dlc(&s).unwrap();
    Ok("")
}

fn get_config(dm: DownloadManager) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::json(&dm.get_config().get()))
}

fn post_config_server(
    dm: DownloadManager,
    json: ::spe3d::config::ConfigServer,
) -> Result<impl warp::Reply, warp::Rejection> {
    dm.get_config().set_server(json).unwrap();
    Ok("")
}

fn post_config_account(
    dm: DownloadManager,
    json: ::spe3d::config::ConfigAccount,
) -> Result<impl warp::Reply, warp::Rejection> {
    dm.get_config().add_account(json).unwrap();
    Ok("")
}

fn delete_config_account(
    id: usize,
    dm: DownloadManager,
) -> Result<impl warp::Reply, warp::Rejection> {
    dm.get_config().remove_account(id).unwrap();
    Ok("")
}
