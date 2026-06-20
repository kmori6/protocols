use axum::{
    Router,
    extract::State,
    response::sse::{Event, KeepAlive, Sse},
    routing::get,
};
use std::{convert::Infallible, time::Duration};
use tokio::sync::broadcast;
use tokio_stream::{
    Stream, StreamExt as _,
    wrappers::{BroadcastStream, IntervalStream},
};

#[derive(Clone)]
pub struct SseState {
    tx: broadcast::Sender<String>,
}

impl SseState {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(64);
        Self { tx }
    }
}

pub fn router() -> Router<SseState> {
    Router::new().route("/ticker", get(ticker_handler)).route(
        "/broadcast",
        get(broadcast_get_handler).post(broadcast_post_handler),
    )
}

// ── ticker ────────────────────────────────────────────────────────────────────

// GET /sse/ticker
// Emits a "tick" event with an incrementing counter every second.
async fn ticker_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let interval = tokio::time::interval(Duration::from_secs(1));
    let mut count = 0;
    let stream = IntervalStream::new(interval).map(move |_| {
        let event = Event::default().event("tick").data(count.to_string());
        count += 1;
        Ok(event)
    });

    Sse::new(stream).keep_alive(KeepAlive::default())
}

// ── broadcast ─────────────────────────────────────────────────────────────────

// GET /sse/broadcast
// Subscribes to the broadcast channel and streams events to the client.
async fn broadcast_get_handler(
    State(state): State<SseState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = state.tx.subscribe();
    let stream = BroadcastStream::new(rx)
        .filter_map(Result::ok)
        .map(|text| Ok(Event::default().event("message").data(text)));

    Sse::new(stream).keep_alive(KeepAlive::default())
}

// POST /sse/broadcast
// Publishes a message to all active SSE subscribers.
async fn broadcast_post_handler(State(state): State<SseState>, body: String) {
    // send() errors only when there are no active receivers, which is not an error
    let _ = state.tx.send(body);
}
