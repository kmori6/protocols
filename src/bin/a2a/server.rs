mod agent;

use a2a_server::{DefaultRequestHandler, InMemoryTaskStore, StaticAgentCard};
use agent::{EchoExecutor, agent_card};
use std::{error::Error, sync::Arc};

const ADDRESS: &str = "127.0.0.1:3003";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let handler = Arc::new(DefaultRequestHandler::new(
        EchoExecutor,
        InMemoryTaskStore::new(),
    ));
    let card = Arc::new(StaticAgentCard::new(agent_card()));
    let router = axum::Router::new()
        .nest("/jsonrpc", a2a_server::jsonrpc::jsonrpc_router(handler))
        .merge(a2a_server::agent_card::agent_card_router(card));
    let listener = tokio::net::TcpListener::bind(ADDRESS).await?;

    println!("A2A server listening on http://{ADDRESS}");
    println!("Agent Card: http://{ADDRESS}/.well-known/agent-card.json");
    println!("JSON-RPC:   http://{ADDRESS}/jsonrpc");
    axum::serve(listener, router).await?;

    Ok(())
}
