use std::thread;
use Config;
use ws;
use spe3d::manager::DownloadManager;
use error::*;
use error_chain::ChainedError;

pub fn start_ws(config: &Config, dm: &DownloadManager) {
    // Create simple websocket that just prints out messages
    let me = ws::WebSocket::new(|_| {
        move |msg| {
            println!("Peer got message: {}", msg);
            Ok(())
        }
    }).unwrap();

    // Get a sender for ALL connections to the websocket
    let broacaster = me.broadcaster();

    let host = format!("{}:{}", config.get().webserver_ip, config.get().websocket_port);
    println!("{:?}", host);

    thread::spawn(move || {
        me.listen(host).unwrap();
    });

    let dm = dm.clone();
    thread::spawn(move || {
        loop {
            if let Err(e) = handle_updates(&broacaster, &dm) {
                println!("{}", e.display_chain().to_string());
            }
        }
    });
}

fn handle_updates(sender: &ws::Sender, dm: &DownloadManager) -> Result<()> {
    let msg = ::serde_json::to_string(&dm.recv_update()?)?;

    //me.listen(host).unwrap();
    sender.send(msg)?;
    Ok(())
}