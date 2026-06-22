mod tools;

use axum::Router;
use rmcp::transport::streamable_http_server::{
    StreamableHttpServerConfig, StreamableHttpService, session::local::LocalSessionManager,
};
use std::error::Error;
use tools::Tools;

const ADDRESS: &str = "127.0.0.1:3002";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let service: StreamableHttpService<Tools, LocalSessionManager> = StreamableHttpService::new(
        || Ok(Tools),
        Default::default(),
        StreamableHttpServerConfig::default(),
    );
    let router = Router::new().nest_service("/mcp", service);
    let listener = tokio::net::TcpListener::bind(ADDRESS).await?;

    println!("MCP Streamable HTTP server listening on http://{ADDRESS}/mcp");
    axum::serve(listener, router).await?;

    Ok(())
}
