use rmcp::{handler::server::wrapper::Parameters, schemars, tool, tool_router};
use serde::Deserialize;

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct EchoParams {
    message: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct AddParams {
    a: i64,
    b: i64,
}

fn echo(message: String) -> String {
    message
}

fn add(a: i64, b: i64) -> i64 {
    a + b
}

#[derive(Clone)]
pub struct Tools;

#[tool_router(server_handler)]
impl Tools {
    #[tool(description = "Return the supplied message")]
    fn echo(&self, Parameters(params): Parameters<EchoParams>) -> String {
        echo(params.message)
    }

    #[tool(description = "Add two integers")]
    fn add(&self, Parameters(params): Parameters<AddParams>) -> String {
        add(params.a, params.b).to_string()
    }
}
