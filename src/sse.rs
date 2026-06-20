use axum::{
    extract::State,
    http::StatusCode,
    response::sse::{Event, KeepAlive, Sse},
    routing::get,
    Router,
};
use futures_util::StreamExt;
use std::{convert::Infallible, sync::Arc, time::Duration};
use tokio::sync::broadcast;
use tokio_stream::wrappers::{BroadcastStream, IntervalStream};

pub struct SseState {
    tx: broadcast::Sender<String>,
}

impl SseState {
    pub fn new() -> Arc<Self> {
        let (tx, _) = broadcast::channel(64);
        Arc::new(Self { tx })
    }
}

pub fn router() -> Router<Arc<SseState>> {
    Router::new()
        .route("/ticker", get(ticker_handler))
        .route("/broadcast", get(broadcast_get_handler).post(broadcast_post_handler))
}

// ── ticker ────────────────────────────────────────────────────────────────────

// GET /sse/ticker
// Emits a "tick" event with an incrementing counter every second.
async fn ticker_handler() -> Sse<impl futures_util::Stream<Item = Result<Event, Infallible>>> {
    let interval = tokio::time::interval(Duration::from_secs(1));
    let stream = IntervalStream::new(interval)
        .enumerate()
        .map(|(i, _)| Ok(Event::default().event("tick").data(i.to_string())));

    Sse::new(stream).keep_alive(KeepAlive::default())
}

// ── broadcast ─────────────────────────────────────────────────────────────────

// GET /sse/broadcast
// Subscribes to the broadcast channel and streams events to the client.
async fn broadcast_get_handler(
    State(state): State<Arc<SseState>>,
) -> Sse<impl futures_util::Stream<Item = Result<Event, Infallible>>> {
    let rx = state.tx.subscribe();
    let stream = BroadcastStream::new(rx)
        .filter_map(|result| async move { result.ok() })
        .map(|text| Ok(Event::default().event("message").data(text)));

    Sse::new(stream).keep_alive(KeepAlive::default())
}

// POST /sse/broadcast
// Publishes a message to all active SSE subscribers.
async fn broadcast_post_handler(
    State(state): State<Arc<SseState>>,
    body: String,
) -> StatusCode {
    // send() errors only when there are no active receivers, which is not an error
    let _ = state.tx.send(body);
    StatusCode::OK
}
