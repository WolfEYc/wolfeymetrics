use tonic::{transport::Server, Request, Response, Status};

use wolfey_metrics::{
    wolfey_metrics_server::{WolfeyMetrics, WolfeyMetricsServer},
    ImportReply, ImportRequest,
};

pub mod wolfey_metrics {
    tonic::include_proto!("wolfeymetrics"); // The string specified here must match the proto package name
}
mod wolfeystorage;
#[derive(Debug, Default)]
pub struct WolfeyMetricsService {}

#[tonic::async_trait]
impl WolfeyMetrics for WolfeyMetricsService {
    async fn import(
        &self,
        request: Request<ImportRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<ImportReply>, Status> {
        // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);
        let req = request.into_inner();

        let reply = ImportReply {
            message: format!("got req {req:?}!"), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = WolfeyMetricsService::default();

    Server::builder()
        .add_service(WolfeyMetricsServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
