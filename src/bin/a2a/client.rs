use a2a::{event::StreamResponse, *};
use a2a_client::{A2AClientFactory, agent_card::AgentCardResolver};
use futures_util::StreamExt;
use std::error::Error;

const SERVER_URL: &str = "http://127.0.0.1:3003";

fn request(text: &str) -> SendMessageRequest {
    SendMessageRequest {
        message: Message::new(Role::User, vec![Part::text(text)]),
        configuration: None,
        metadata: None,
        tenant: None,
    }
}

fn print_task(task: &Task) {
    println!("task: {} {:?}", task.id, task.status.state);
    if let Some(message) = &task.status.message
        && let Some(text) = message.text()
    {
        println!("{text}");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let card = AgentCardResolver::new(None).resolve(SERVER_URL).await?;
    println!("Agent: {}", card.name);
    println!("Skills:");
    for skill in &card.skills {
        println!("- {}", skill.id);
    }

    let client = A2AClientFactory::builder()
        .preferred_bindings(vec![TRANSPORT_PROTOCOL_JSONRPC.to_string()])
        .build()
        .create_from_card(&card)
        .await?;

    println!("\nSendMessage:");
    let task_id = match client.send_message(&request("hello")).await? {
        SendMessageResponse::Task(task) => {
            print_task(&task);
            task.id
        }
        SendMessageResponse::Message(message) => {
            println!("message: {}", message.text().unwrap_or("(no text)"));
            client.destroy().await?;
            return Ok(());
        }
    };

    println!("\nGetTask:");
    let task = client
        .get_task(&GetTaskRequest {
            id: task_id,
            history_length: Some(10),
            tenant: None,
        })
        .await?;
    print_task(&task);

    println!("\nSendStreamingMessage:");
    let mut stream = client
        .send_streaming_message(&request("stream hello"))
        .await?;
    let mut index = 0;
    while let Some(event) = stream.next().await {
        index += 1;
        match event? {
            StreamResponse::Task(task) => {
                println!("{index}: task {:?}", task.status.state);
                if let Some(message) = &task.status.message
                    && let Some(text) = message.text()
                {
                    println!("{text}");
                }
            }
            StreamResponse::StatusUpdate(update) => {
                println!("{index}: status {:?}", update.status.state);
            }
            StreamResponse::Message(message) => {
                println!("{index}: message {}", message.text().unwrap_or("(no text)"));
            }
            StreamResponse::ArtifactUpdate(update) => {
                println!("{index}: artifact {}", update.artifact.artifact_id);
            }
        }
    }

    client.destroy().await?;

    Ok(())
}
