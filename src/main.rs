mod rest;

use std::{collections::HashMap, sync::{Arc, Mutex}};

#[tokio::main]
async fn main() {
    let db: rest::Db = Arc::new(Mutex::new(HashMap::new()));

    let app = rest::router().with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
