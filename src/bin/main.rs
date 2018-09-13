#[macro_use] extern crate error_chain;
extern crate serde;
extern crate serde_json;
extern crate toml;
extern crate actix_web;
extern crate spe3d;
extern crate ws;
extern crate futures;
extern crate bytes;

pub mod error;
pub mod websocket;

use spe3d::config::Config;
use spe3d::models::DownloadPackage;
use spe3d::DownloadManager;

use actix_web::{
    fs::StaticFiles, http, server, App, HttpRequest, HttpResponse,
    Json, HttpMessage, FutureResponse, AsyncResponder, Result
};
use futures::future::Future;
use bytes::Bytes;
use actix_web::middleware::cors::Cors;

fn main() {
    // load the config file
    let config = Config::new();

    // create the download manager
    let dm = DownloadManager::new(config.clone().into()).unwrap();
    dm.start();

    // test ecke
    let so = ::spe3d::loader::so::ShareOnline::new(config.clone());
    //so.update_account(&mut config.get().get_first_so().unwrap());
    //so.check_url("http://www.share-online.biz/dl/NPE1KXEPFQM").unwrap();

    // start the websocket server and add it to the download manager
    websocket::start_ws(&config, &dm);

    // start the actix webserver
    server::new(move || {
        vec![
            App::with_state(dm.clone())
                .middleware(Cors::build().send_wildcard().finish())
                .resource("/api/test", |r| r.method(http::Method::GET).with(api_test))
                .resource("/api/downloads", |r| r.method(http::Method::GET).with(api_downloads))
                .resource("/api/start-download/{id}", |r| r.method(http::Method::POST).with(api_start_download))
                .resource("/api/add-links", |r| r.method(http::Method::POST).with(api_add_links))
                .resource("/api/delete-link/{id}", |r| r.method(http::Method::POST).with(api_remove_link))
                .resource("/api/add-dlc", |r| r.method(http::Method::POST).with(api_add_dlc))
                .resource("/api/config", |r| r.method(http::Method::GET).with(api_config))
                .resource("/api/config/server", |r| r.method(http::Method::POST).with(post_config_server))
                .resource("/api/config/account", |r| r.method(http::Method::POST).with(post_config_account))
                .resource("/api/config/account/{id}", |r| r.method(http::Method::DELETE).with(delete_config_account))
                .handler("/", StaticFiles::new("www").unwrap().index_file("index.html"))
                .finish(),
        ]
    }).bind("0.0.0.0:8000")
    .unwrap()
    .run();
}

fn api_test(_req: HttpRequest<DownloadManager>) -> Result<&'static str> {
    Ok("Success")
}

fn api_downloads(req: HttpRequest<DownloadManager>) -> Result<Json<Vec<DownloadPackage>>> {
    Ok(Json(req.state().get_downloads().unwrap()))
}

fn api_start_download(req: HttpRequest<DownloadManager>) -> Result<String> {
    let id: usize = req.match_info().query("id")?;
    req.state().start_download(id).unwrap();
    Ok("".to_string())
}

fn api_add_links(req: HttpRequest<DownloadManager>, json: Json<serde_json::Value>) -> Result<String> {
    // add the links as a package
    req.state().add_links(
        // get the name
        json["name"]
            .as_str()
            .ok_or("Package name is not provided").unwrap(),
        // get the links
        json["links"]
            .as_array()
            .ok_or("Package links are not provided").unwrap()
            .iter()
            .map(|u| u.as_str())
            .filter(|u| u.is_some())
            .map(|u| u.unwrap().to_string())
            .collect(),
    ).unwrap();
    Ok("".to_string())
}

fn api_remove_link(req: HttpRequest<DownloadManager>) -> Result<String> {
    let id: usize = req.match_info().query("id")?;
    // remove the container or link
    req.state().remove(id).unwrap();
    Ok("".to_string())
}

fn api_add_dlc(req: HttpRequest<DownloadManager>) -> FutureResponse<HttpResponse> {
    req.body()                     // <- get Body future
       .limit(1_000_000)           // <- change max size of the body to a 1mb
       .from_err()
       .and_then(move |bytes: Bytes| {  // <- complete body
            req.state().add_dlc(::std::str::from_utf8(&bytes).unwrap()).unwrap();

            Ok(HttpResponse::Ok().into())
       }).responder()
}

fn api_config(req: HttpRequest<DownloadManager>) -> Json<::spe3d::config::ConfigData> {
    Json(req.state().get_config().get())
}

fn post_config_server(req: HttpRequest<DownloadManager>, json: Json<::spe3d::config::ConfigServer>) -> Result<String> {
    req.state().get_config().set_server(json.into_inner()).unwrap();    
    Ok("".to_string())
}

fn post_config_account(req: HttpRequest<DownloadManager>, json: Json<::spe3d::config::ConfigAccount>) -> Result<String> {
    req.state().get_config().add_account(json.into_inner()).unwrap();    
    Ok("".to_string())
}

fn delete_config_account(req: HttpRequest<DownloadManager>) -> Result<String> {
    let id: usize = req.match_info().query("id")?;
    req.state().get_config().remove_account(id).unwrap();
    Ok("".to_string())
}