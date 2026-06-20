use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
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
        .route(
            "/items/{id}",
            get(get_item).put(update_item).delete(delete_item),
        )
}

// GET /items
async fn list_items(State(db): State<Db>) -> Json<Vec<Item>> {
    let db = db.lock().unwrap();
    Json(db.values().cloned().collect())
}

// POST /items
async fn create_item(
    State(db): State<Db>,
    Json(payload): Json<ItemPayload>,
) -> (StatusCode, Json<Item>) {
    let mut db = db.lock().unwrap();
    let id = db.keys().max().copied().unwrap_or(0) + 1;
    let item = Item {
        id,
        name: payload.name,
    };
    db.insert(id, item.clone());
    (StatusCode::CREATED, Json(item))
}

// GET /items/:id
async fn get_item(State(db): State<Db>, Path(id): Path<u32>) -> Result<Json<Item>, StatusCode> {
    let db = db.lock().unwrap();
    db.get(&id).cloned().map(Json).ok_or(StatusCode::NOT_FOUND)
}

// PUT /items/:id
async fn update_item(
    State(db): State<Db>,
    Path(id): Path<u32>,
    Json(payload): Json<ItemPayload>,
) -> Result<Json<Item>, StatusCode> {
    let mut db = db.lock().unwrap();
    let item = db.get_mut(&id).ok_or(StatusCode::NOT_FOUND)?;
    item.name = payload.name;
    Ok(Json(item.clone()))
}

// DELETE /items/:id
async fn delete_item(State(db): State<Db>, Path(id): Path<u32>) -> StatusCode {
    let mut db = db.lock().unwrap();
    if db.remove(&id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
