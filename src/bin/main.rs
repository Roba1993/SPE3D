extern crate bytes;
extern crate futures;
extern crate serde;
extern crate serde_json;
extern crate spe3d;
extern crate warp;

use spe3d::config::Config;
use spe3d::DownloadManager;

use bytes::Buf;
use futures::stream::SplitSink;
use futures::{Future, Sink, Stream};
use std::collections::HashMap;
use std::io::Read;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Mutex,
};
use warp::ws::{Message, WebSocket};
use warp::Filter;

fn main() {
    // load the config file
    let config = Config::new();

    // create the download manager
    let dm = DownloadManager::new(config.clone().into()).unwrap();
    dm.start();

    // start the websocket server and add it to the download manager
    //websocket::start_ws(&config, &dm);

    // Keep track of all connected users, key is usize, value
    // is a websocket sender.
    let users = Arc::new(Mutex::new(HashMap::new()));
    handle_dm_updates(&dm, &users);
    let users = warp::any().map(move || users.clone());


    // warp webserver code
    let dmw = dm.clone();
    let dmw = warp::any().map(move || dmw.clone());

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
        .and(dmw.clone())
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
        .and(dmw.clone())
        .and_then(post_start_download)
        .with(warp::reply::with::header(
            "Access-Control-Allow-Origin",
            "*",
        ));

    let post_add_links = warp::post2()
        .and(api.and(warp::path("add-links")).and(warp::path::index()))
        .and(dmw.clone())
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
        .and(dmw.clone())
        .and_then(post_delete_link)
        .with(warp::reply::with::header(
            "Access-Control-Allow-Origin",
            "*",
        ));

    let post_add_dlc = warp::post2()
        .and(api.and(warp::path("add-dlc")).and(warp::path::index()))
        .and(dmw.clone())
        .and(warp::body::concat())
        .and_then(post_add_dlc)
        .with(warp::reply::with::header(
            "Access-Control-Allow-Origin",
            "*",
        ));

    let get_config = warp::get2()
        .and(api.and(warp::path("config")).and(warp::path::index()))
        .and(dmw.clone())
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
        .and(dmw.clone())
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
        .and(dmw.clone())
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
        .and(dmw.clone())
        .and_then(delete_config_account)
        .with(warp::reply::with::header(
            "Access-Control-Allow-Origin",
            "*",
        ));

    let options = warp::options()
        .map(warp::reply)
        .with(warp::reply::with::header(
            "Access-Control-Allow-Origin",
            "*",
        ))
        .with(warp::reply::with::header(
            "Access-Control-Allow-Headers",
            "*",
        ));

    let ws_updates = warp::path("updates")
        // The `ws2()` filter will prepare Websocket handshake...
        .and(warp::ws2())
        .and(users)
        .map(|ws: warp::ws::Ws2, users| {
            // This will call our function if the handshake succeeds.
            ws.on_upgrade(move |socket| {
                ws_user_connected(socket, users)
            })
        });

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
        .or(delete_config_account)
        .or(ws_updates)
        .or(options);

    warp::serve(routes).run(dm.get_config().get().get_server_addr().unwrap());
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
    Ok("as")
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

/********* Websocket **********/
/// Our global unique user id counter.
static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);

/// Our state of currently connected users.
///
/// - Key is their id
/// - Value is a sender of `warp::ws::Message`s
type Users = Arc<Mutex<HashMap<usize, SplitSink<WebSocket>>>>;

fn ws_user_connected(
    ws: WebSocket,
    users: Users,
) -> impl Future<Item = (), Error = ()> {
    // Use a counter to assign a new unique ID for this user.
    let my_id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);

    // Split the socket into a sender and receive of messages.
    let (tx, user_ws_rx) = ws.split();

    // Save the sender in our list of connected users.
    users.lock().unwrap().insert(my_id, tx);

    // Return a `Future` that is basically a state machine managing
    // this specific user's connection.

    // Make an extra clone to give to our disconnection handler...
    let users2 = users.clone();

    user_ws_rx
        // Every time the user sends a message, do nothing...
        .for_each(move |_| {
            //ws_user_message(my_id, msg, &users);
            Ok(())
        })
        // for_each will keep processing as long as the user stays
        // connected. Once they disconnect, then remove from list
        .then(move |result| {
            &users2.lock().unwrap().remove(&my_id);
            result
        })
        // If at any time, there was a websocket error, log here...
        .map_err(move |e| {
            eprintln!("websocket error(uid={}): {}", my_id, e);
        })
}

pub fn handle_dm_updates(dm: &DownloadManager, users: &Users) {
    let dm = dm.clone();
    let users = users.clone();

    ::std::thread::spawn(move || loop {
        if let Err(e) = handle_updates(&users, &dm) {
            println!("{}", e.to_string());
        }
    });
}

fn handle_updates(users: &Users, dm: &DownloadManager) -> Result<(), ::spe3d::error::Error> {
    let msg = ::serde_json::to_string(&dm.recv_update()?)?;

    for (_, tx) in users.lock().unwrap().iter_mut() {
        let _ = tx.start_send(Message::text(msg.clone()));
    }

    Ok(())
}
