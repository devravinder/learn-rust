// proj_04_grpc — a gRPC service with tonic (contrast to the Axum REST project).
//
// Run the server:  cargo run --bin proj_04_grpc
// Self-test (spawns server + calls it): cargo run --bin proj_04_grpc -- --selftest
//
// gRPC differs from REST: a typed .proto contract, HTTP/2 transport, binary
// protobuf payloads, and generated client/server stubs (see build.rs + proto/).

// Pull in the code generated from proto/greeter.proto.
pub mod greeter {
    tonic::include_proto!("greeter");
}

use greeter::greeter_server::{Greeter, GreeterServer};
use greeter::{HelloReply, HelloRequest};
use greeter::greeter_client::GreeterClient;
use std::env;
use tonic::{Request, Response, Status, transport::Server};

// Our service implementation.
#[derive(Default)]
struct MyGreeter;

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let name = request.into_inner().name;
        Ok(Response::new(HelloReply {
            message: format!("Hello, {name}!"),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051";

    if env::args().any(|a| a == "--selftest") {
        return selftest(addr).await;
    }

    println!("gRPC Greeter listening on {addr}");
    Server::builder()
        .add_service(GreeterServer::new(MyGreeter))
        .serve(addr.parse()?)
        .await?;
    Ok(())
}

// Start the server in the background, then call it with a generated client.
async fn selftest(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let bind = addr.to_string();
    tokio::spawn(async move {
        Server::builder()
            .add_service(GreeterServer::new(MyGreeter))
            .serve(bind.parse().unwrap())
            .await
            .unwrap();
    });
    // Give the server a moment to start.
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    let mut client = GreeterClient::connect(format!("http://{addr}")).await?;
    let reply = client
        .say_hello(Request::new(HelloRequest { name: "Rustacean".into() }))
        .await?;
    println!("selftest reply: {}", reply.into_inner().message);
    Ok(())
}
