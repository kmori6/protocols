mod tools;

use rmcp::{ServiceExt, transport::stdio};
use std::error::Error;
use tools::Tools;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let service = Tools.serve(stdio()).await?;
    service.waiting().await?;

    Ok(())
}
