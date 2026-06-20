mod rest;
mod ws;

use std::{collections::HashMap, sync::{Arc, Mutex}};

#[tokio::main]
async fn main() {
    let rest_db: rest::Db = Arc::new(Mutex::new(HashMap::new()));
    let ws_state = ws::WsState::new();

    let app = rest::router()
        .with_state(rest_db)
        .merge(ws::router().with_state(ws_state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
