use std::thread;
use ws;


pub fn start_ws() -> ws::Sender {
    // Create simple websocket that just prints out messages
    let me = ws::WebSocket::new(|_| {
        move |msg| {
            Ok(println!("Peer got message: {}", msg))
        }
    }).unwrap();

    // Get a sender for ALL connections to the websocket
    let broacaster = me.broadcaster();

    thread::spawn(move || {
        me.listen("127.0.0.1:8001").unwrap();
    });

    broacaster
}