use rmcp::{ServiceExt, model::CallToolRequestParams, transport::StreamableHttpClientTransport};
use serde_json::json;
use std::error::Error;

const SERVER_URL: &str = "http://127.0.0.1:3002/mcp";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let transport = StreamableHttpClientTransport::from_uri(SERVER_URL);
    let client = ().serve(transport).await?;

    println!("Available tools:");
    for tool in client.list_all_tools().await? {
        println!("- {}", tool.name);
    }

    let arguments = json!({ "a": 2, "b": 3 })
        .as_object()
        .expect("tool arguments must be an object")
        .clone();
    let result = client
        .call_tool(CallToolRequestParams::new("add").with_arguments(arguments))
        .await?;

    for content in result.content {
        if let Some(text) = content.as_text() {
            println!("add(2, 3) = {}", text.text);
        }
    }

    client.cancel().await?;

    Ok(())
}
