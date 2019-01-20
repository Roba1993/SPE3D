use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use crate::error::*;
use crate::models::{DownloadFile, DownloadList, CaptchaResult};

/// Message bus to share messages through the
/// complete system.
#[derive(Clone)]
pub struct MessageBus {
    sender: Arc<Mutex<Sender<Message>>>,
    receiver_internal: Arc<Mutex<Receiver<Message>>>,
    sender_internal: Arc<Mutex<Vec<Sender<Message>>>>,
}

impl MessageBus {
    /// Create a new Message Bus
    pub fn new() -> MessageBus {
        let (sender, receiver) = channel();

        let bus = MessageBus {
            sender: Arc::new(Mutex::new(sender)),
            receiver_internal: Arc::new(Mutex::new(receiver)),
            sender_internal: Arc::new(Mutex::new(vec!())),
        };

        // a clone from the msg bus for the internal handler
        let bus_internal = bus.clone();
        // new thread for the bus handler
        thread::spawn(move || loop {
            match bus_internal.handle_msg() {
                Ok(_) => {}
                Err(e) => println!("{}", e),
            }
        });

        bus
    }

    /// Internal function to handle the incoming messages
    /// and send them to all receivers
    fn handle_msg(&self) -> Result<()> {
        let msg = self.receiver_internal.lock()?.recv()?;

        let mut senders = self.sender_internal.lock()?;
        for is in 0..senders.len() {
            if let Err(_) = senders.get(is).ok_or("Sender was't in list")?.send(msg.clone()) {
                senders.remove(is);
            }
        }

        Ok(())
    }

    /// Create a channel to the bus to send messages to all receivers
    /// and a receiver to get all messages
    pub fn channel(&self) -> Result<(Sender<Message>, Receiver<Message>)> {
        let (sender, receiver) = channel();

        self.sender_internal.lock()?.push(sender);

        Ok((self.sender.lock()?.clone(), receiver))
    }

    /// Get a sender to the bus
    pub fn get_sender(&self) -> Result<Sender<Message>> {
        Ok(self.sender.lock()?.clone())
    }
}

/// Message to be send over the Message bus
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum Message {
    // Complete list of all download files
    DownloadList(DownloadList),
    // DownloadSpeed((file_id, speed per sec))
    DownloadSpeed((usize, usize)),
    // Request a captcha solving
    CaptchaRequest(DownloadFile),
    // Response with download url
    CaptchaResponse(CaptchaResult)
}

impl Message {
    /// Returns the captcha response or an None
    pub fn get_captcha_response(&self) -> Option<&CaptchaResult> {
        match self {
            Message::CaptchaResponse(v) => Some(v),
            _ => None
        }
    }
}