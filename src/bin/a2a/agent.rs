use a2a::{event::StreamResponse, *};
use a2a_server::{AgentExecutor, ExecutorContext};
use futures_util::{stream, stream::BoxStream};
use std::time::Duration;
use tokio_stream::wrappers::ReceiverStream;

pub struct EchoExecutor;

impl AgentExecutor for EchoExecutor {
    fn execute(
        &self,
        context: ExecutorContext,
    ) -> BoxStream<'static, Result<StreamResponse, A2AError>> {
        let message = context.message;
        let task_id = context.task_id;
        let context_id = context.context_id;
        let input = message
            .as_ref()
            .and_then(Message::text)
            .unwrap_or("(no text)")
            .to_string();

        let working = StreamResponse::StatusUpdate(TaskStatusUpdateEvent {
            task_id: task_id.clone(),
            context_id: context_id.clone(),
            status: TaskStatus {
                state: TaskState::Working,
                message: None,
                timestamp: None,
            },
            metadata: None,
        });

        let completed = StreamResponse::Task(Task {
            id: task_id.clone(),
            context_id: context_id.clone(),
            status: TaskStatus {
                state: TaskState::Completed,
                message: Some(Message {
                    role: Role::Agent,
                    message_id: new_message_id(),
                    task_id: Some(task_id),
                    context_id: Some(context_id),
                    parts: vec![Part::text(format!("Echo: {input}"))],
                    metadata: None,
                    extensions: None,
                    reference_task_ids: None,
                }),
                timestamp: None,
            },
            artifacts: None,
            history: None,
            metadata: None,
        });

        let (sender, receiver) = tokio::sync::mpsc::channel(2);
        tokio::spawn(async move {
            let _ = sender.send(Ok(working)).await;
            tokio::time::sleep(Duration::from_secs(1)).await;
            let _ = sender.send(Ok(completed)).await;
        });

        Box::pin(ReceiverStream::new(receiver))
    }

    fn cancel(
        &self,
        context: ExecutorContext,
    ) -> BoxStream<'static, Result<StreamResponse, A2AError>> {
        Box::pin(stream::once(async move {
            Ok(StreamResponse::StatusUpdate(TaskStatusUpdateEvent {
                task_id: context.task_id,
                context_id: context.context_id,
                status: TaskStatus {
                    state: TaskState::Canceled,
                    message: None,
                    timestamp: None,
                },
                metadata: None,
            }))
        }))
    }
}

pub fn agent_card() -> AgentCard {
    AgentCard {
        name: "Echo Agent".to_string(),
        description: "An A2A agent that echoes text messages.".to_string(),
        version: VERSION.to_string(),
        supported_interfaces: vec![AgentInterface::new(
            "http://127.0.0.1:3003/jsonrpc",
            TRANSPORT_PROTOCOL_JSONRPC,
        )],
        capabilities: AgentCapabilities {
            streaming: Some(true),
            push_notifications: Some(false),
            extensions: None,
            extended_agent_card: None,
        },
        default_input_modes: vec!["text/plain".to_string()],
        default_output_modes: vec!["text/plain".to_string()],
        skills: vec![AgentSkill {
            id: "echo".to_string(),
            name: "Echo".to_string(),
            description: "Echoes the supplied text message.".to_string(),
            tags: vec!["echo".to_string()],
            examples: Some(vec!["hello".to_string()]),
            input_modes: Some(vec!["text/plain".to_string()]),
            output_modes: Some(vec!["text/plain".to_string()]),
            security_requirements: None,
        }],
        provider: None,
        documentation_url: None,
        icon_url: None,
        security_schemes: None,
        security_requirements: None,
        signatures: None,
    }
}
