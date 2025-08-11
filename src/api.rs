use tonic::{transport::Server, Request, Response, Status};
use proto::rustualize_server::{Rustualize, RustualizeServer};
use proto::{RustualizeRequest, RustualizeResponse};

pub mod proto {
    tonic::include_proto!("rustualize");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("rustualize_descriptor");
}

#[derive(Debug, Default)]
pub struct RustualizeApi {}

#[tonic::async_trait]
impl Rustualize for RustualizeApi {
    async fn start(&self, request: Request<RustualizeRequest>) -> Result<Response<RustualizeResponse>, Status> {
        println!("Got a request {:?}", request);

        let response = RustualizeResponse {
            code: "OK BRO".to_string(),
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse()?;
    let rustualize = RustualizeApi::default();
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build_v1()
        .unwrap();

    Server::builder()
        .add_service(reflection_service)
        .add_service(RustualizeServer::new(rustualize))
        .serve(addr)
        .await?;

    Ok(())
}
