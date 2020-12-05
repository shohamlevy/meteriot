use tonic::{transport::Server, Request, Response, Status};

use meteriot_base::say_server::{Say, SayServer};
use meteriot_base::{SayRequest, SayResponse};
mod meteriot_base;

#[derive(Default)]
pub struct MySay {}

#[tonic::async_trait]
impl Say for MySay {
    async fn send(&self, requst: Request<SayRequest>) -> Result<Response<SayResponse>, Status> {
        Ok(Response::new(SayResponse {
            message: format!("hello {}!", requst.get_ref().name),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let say = MySay::default();
    println!("Server is listening on address: {}", addr);

    Server::builder()
        .add_service(SayServer::new(say))
        .serve(addr)
        .await?;
    Ok(())
}
