use actix_web::{
    App, HttpServer,
    web::{self},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::sync::mpsc;

use crate::{
    controllers::v1::{create_limit_order, index, sign_in, sign_up},
    engine::run_engine,
};
pub mod engine;

pub mod controllers {
    pub mod v1;
}
// user struct
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    username: String,
    name: String,
    password: String,
    age: u8,
}
// we need this here so every one can get a clone of it;
#[derive(Clone, Debug)]
pub struct AppState {
    users: Arc<Mutex<HashMap<String, User>>>, // hashmap will have key of usename and value will be user details
    session_ids: Arc<Mutex<HashMap<String, String>>>,
    trades_sender: mpsc::Sender<String>, // type of order.
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (sender, receiver) = mpsc::channel(100);
    let state = web::Data::new(AppState {
        users: Arc::new(Mutex::new(HashMap::new())),
        session_ids: Arc::new(Mutex::new(HashMap::new())),
        trades_sender: sender,
    });

    tokio::spawn(run_engine(receiver));

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/", web::get().to(index))
            .route("/signup", web::post().to(sign_up))
            .route("/signin", web::post().to(sign_in))
            .route("/create_limit_order", web::post().to(create_limit_order))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
