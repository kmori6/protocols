mod generated;

use generated::{
    AddRequest, AddResponse, ChatMessage, CountRequest, CountResponse, NumberRequest, SumResponse,
    demo_service_server::{DemoService, DemoServiceServer},
};
use std::{error::Error, net::SocketAddr};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, Streaming, transport::Server};

#[derive(Default)]
struct Demo;

#[tonic::async_trait]
impl DemoService for Demo {
    async fn add(&self, request: Request<AddRequest>) -> Result<Response<AddResponse>, Status> {
        let request = request.into_inner();
        Ok(Response::new(AddResponse {
            result: request.a + request.b,
        }))
    }

    type CountUpStream = ReceiverStream<Result<CountResponse, Status>>;

    async fn count_up(
        &self,
        request: Request<CountRequest>,
    ) -> Result<Response<Self::CountUpStream>, Status> {
        let count = request.into_inner().count;
        if count < 1 {
            return Err(Status::invalid_argument("count must be positive"));
        }

        let (sender, receiver) = mpsc::channel(4);
        tokio::spawn(async move {
            for value in 1..=count {
                if sender.send(Ok(CountResponse { value })).await.is_err() {
                    break;
                }
                tokio::time::sleep(std::time::Duration::from_millis(200)).await;
            }
        });

        Ok(Response::new(ReceiverStream::new(receiver)))
    }

    async fn sum(
        &self,
        request: Request<Streaming<NumberRequest>>,
    ) -> Result<Response<SumResponse>, Status> {
        let mut stream = request.into_inner();
        let mut result = 0;

        while let Some(number) = stream.message().await? {
            result += number.value;
        }

        Ok(Response::new(SumResponse { result }))
    }

    type ChatStream = ReceiverStream<Result<ChatMessage, Status>>;

    async fn chat(
        &self,
        request: Request<Streaming<ChatMessage>>,
    ) -> Result<Response<Self::ChatStream>, Status> {
        let mut stream = request.into_inner();
        let (sender, receiver) = mpsc::channel(4);

        tokio::spawn(async move {
            loop {
                match stream.message().await {
                    Ok(Some(message)) => {
                        let response = ChatMessage {
                            text: format!("Echo: {}", message.text),
                        };
                        if sender.send(Ok(response)).await.is_err() {
                            break;
                        }
                    }
                    Ok(None) => break,
                    Err(status) => {
                        let _ = sender.send(Err(status)).await;
                        break;
                    }
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(receiver)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let address = "127.0.0.1:50051".parse::<SocketAddr>()?;
    println!("gRPC server listening on http://{address}");

    Server::builder()
        .add_service(DemoServiceServer::new(Demo))
        .serve(address)
        .await?;

    Ok(())
}
