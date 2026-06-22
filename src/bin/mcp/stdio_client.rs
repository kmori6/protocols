use rmcp::{ServiceExt, model::CallToolRequestParams, transport::TokioChildProcess};
use serde_json::json;
use std::{env, error::Error};
use tokio::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server_path = env::current_exe()?.with_file_name("mcp_stdio_server");
    let transport = TokioChildProcess::new(Command::new(server_path))?;
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
