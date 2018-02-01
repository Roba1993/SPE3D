use std::thread;
use config::Config;
use ws;


pub fn start_ws(config: Config) -> ws::Sender {
    // Create simple websocket that just prints out messages
    let me = ws::WebSocket::new(|_| {
        move |msg| {
            Ok(println!("Peer got message: {}", msg))
        }
    }).unwrap();

    // Get a sender for ALL connections to the websocket
    let broacaster = me.broadcaster();

    let host = format!("{}:{}", config.get().webserver_ip, config.get().websocket_port);

    thread::spawn(move || {
        me.listen(host).unwrap();
    });

    broacaster
}