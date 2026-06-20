use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    id: u32,
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct ItemPayload {
    name: String,
}

pub type Db = Arc<Mutex<HashMap<u32, Item>>>;

pub fn router() -> Router<Db> {
    Router::new()
        .route("/items", get(list_items).post(create_item))
        .route("/items/{id}", get(get_item).put(update_item).delete(delete_item))
}

// GET /items
async fn list_items(State(db): State<Db>) -> (StatusCode, Json<Vec<Item>>) {
    let db = db.lock().unwrap();
    (StatusCode::OK, Json(db.values().cloned().collect()))
}

// POST /items
async fn create_item(
    State(db): State<Db>,
    Json(payload): Json<ItemPayload>,
) -> (StatusCode, Json<Item>) {
    let mut db = db.lock().unwrap();
    let id = db.keys().max().copied().unwrap_or(0) + 1;
    let item = Item { id, name: payload.name };
    db.insert(id, item.clone());
    (StatusCode::CREATED, Json(item))
}

// GET /items/:id
async fn get_item(
    State(db): State<Db>,
    Path(id): Path<u32>,
) -> (StatusCode, Json<Option<Item>>) {
    let db = db.lock().unwrap();
    match db.get(&id).cloned() {
        Some(item) => (StatusCode::OK, Json(Some(item))),
        None => (StatusCode::NOT_FOUND, Json(None)),
    }
}

// PUT /items/:id
async fn update_item(
    State(db): State<Db>,
    Path(id): Path<u32>,
    Json(payload): Json<ItemPayload>,
) -> (StatusCode, Json<Option<Item>>) {
    let mut db = db.lock().unwrap();
    if let Some(item) = db.get_mut(&id) {
        item.name = payload.name;
        (StatusCode::OK, Json(Some(item.clone())))
    } else {
        (StatusCode::NOT_FOUND, Json(None))
    }
}

// DELETE /items/:id
async fn delete_item(
    State(db): State<Db>,
    Path(id): Path<u32>,
) -> StatusCode {
    let mut db = db.lock().unwrap();
    if db.remove(&id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
