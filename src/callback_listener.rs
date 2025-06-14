use axum::{Extension, Json, Router, routing::get};

use std::sync::Arc;
use tokio::sync::mpsc::*;

use serde::{Deserialize, Serialize};

use tracing::{Level, info};
use tracing_subscriber::{self};

pub async fn callback_listener(sender: Sender<Event>) {
    // initialize tracing for logging

    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    let app_state = Arc::new(AppState { tx: sender });
    // build our application with a route

    let app = Router::new()
        //.with_state(state.clone())
        // `GET /` goes to `root`
        .route("/", get(root).post(value_callback))
        .layer(Extension(app_state));
    //.with_state(app_state);

    // run our app with hyper, listening globally on port 5000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    info!("Server is running on http://0.0.0.0:5000");
    axum::serve(listener, app).await.unwrap();
}

// handler for GET /
async fn root() -> &'static str {
    "Hello, world!"
}

// async fn value_callback(State(state): State<&X1>, Json(new_post): Json<ValueCallback>) {
async fn value_callback(state: Extension<Arc<AppState>>, Json(new_post): Json<ValueCallback>) {
    for evt in new_post.events {
        //let _ = state.tx.lock().await.send(evt);
        let _ = state.tx.send(evt).await;
    }
}

pub async fn handle_evt(mut rx: tokio::sync::mpsc::Receiver<Event>) {
    loop {
        if let Ok(evt) = rx.try_recv() {
            println!("New Event: {evt:?}!");
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
struct ValueCallback {
    token: String,
    events: Vec<Event>,
    failtures: Option<u16>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    uid: String,
    value: String,
}

#[derive(Clone, Debug)]
pub struct AppState {
    tx: Sender<Event>,
}
