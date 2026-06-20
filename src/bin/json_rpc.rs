use jsonrpsee::{
    core::RpcResult,
    server::{RpcModule, Server},
    types::Params,
};
use serde::Deserialize;
use serde_json::Value;
use std::{error::Error, net::SocketAddr};

#[derive(Deserialize)]
struct AddParams {
    a: i64,
    b: i64,
}

fn echo(params: Params<'_>) -> RpcResult<Value> {
    params.parse()
}

fn add(params: Params<'_>) -> RpcResult<i64> {
    let params: AddParams = params.parse()?;
    Ok(params.a + params.b)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let address = "0.0.0.0:3001".parse::<SocketAddr>()?;
    let server = Server::builder().build(address).await?;

    let mut module = RpcModule::new(());
    module.register_method("echo", |params, _, _| echo(params))?;
    module.register_method("add", |params, _, _| add(params))?;

    println!("JSON-RPC server listening on {}", server.local_addr()?);

    let handle = server.start(module);
    handle.stopped().await;

    Ok(())
}
