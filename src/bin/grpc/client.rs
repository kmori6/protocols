mod generated;

use generated::{
    AddRequest, ChatMessage, CountRequest, NumberRequest, demo_service_client::DemoServiceClient,
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = DemoServiceClient::connect("http://127.0.0.1:50051").await?;

    let add = client.add(AddRequest { a: 2, b: 3 }).await?.into_inner();
    println!("Unary Add: 2 + 3 = {}", add.result);

    println!("Server Streaming CountUp:");
    let mut count_stream = client
        .count_up(CountRequest { count: 3 })
        .await?
        .into_inner();
    while let Some(message) = count_stream.message().await? {
        println!("- {}", message.value);
    }

    let numbers = tokio_stream::iter([
        NumberRequest { value: 1 },
        NumberRequest { value: 2 },
        NumberRequest { value: 3 },
    ]);
    let sum = client.sum(numbers).await?.into_inner();
    println!("Client Streaming Sum: 1 + 2 + 3 = {}", sum.result);

    println!("Bidirectional Streaming Chat:");
    let messages = tokio_stream::iter([
        ChatMessage {
            text: "hello".to_string(),
        },
        ChatMessage {
            text: "gRPC".to_string(),
        },
    ]);
    let mut chat_stream = client.chat(messages).await?.into_inner();
    while let Some(message) = chat_stream.message().await? {
        println!("- {}", message.text);
    }

    Ok(())
}
