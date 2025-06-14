use crate::callback_listener::Event;
use gira_iot_api::{callback_listener, x1::X1};

use tokio::sync::mpsc;
use tokio::sync::mpsc::*;

#[tokio::main]
async fn main() {
    let myx1 = X1::new("10.10.1.12", "Username", "My$up3rs3cur3P4$$w0rd");
    myx1.connect().await;

    let (tx, mut rx): (Sender<Event>, Receiver<Event>) = mpsc::channel(32);

    println!("{:?}", myx1.get_token());

    //let listener = callback_listener(tx);
    //let (tx, rx): (Sender<Event>, Receiver<Event>) = mpsc::channel();
    //let a = myx1.functions.functions.lock().await;

    tokio::spawn(async move { callback_listener::callback_listener(tx).await });
    loop {
        if let Ok(evt) = rx.try_recv() {
            println!("New Event: {evt:?}!");
        }
    }
}
