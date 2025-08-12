use tonic::{transport::Server, Request, Response, Status};
use proto::rustualize_server::{Rustualize, RustualizeServer};
use proto::{StartContainerRequest, StartContainerResponse};

mod container;
mod cli;
mod errors;
mod config;
mod ipc;
mod child;
mod hostname;
mod mounts;
mod namespaces;
mod capabilities;
mod syscalls;
mod resources;
mod filesystem;

pub mod proto {
    tonic::include_proto!("rustualize");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("rustualize_descriptor");
}

#[derive(Debug, Default)]
pub struct RustualizeApi {}

#[tonic::async_trait]
impl Rustualize for RustualizeApi {
    async fn start(&self, request: Request<StartContainerRequest>) -> Result<Response<StartContainerResponse>, Status> {
        println!("Got a request {:?}", request);

        let res = container::start(request.into_inner());
        let response;

        match res {
            Ok(_) => {
                response = StartContainerResponse {
                    code: "Exit without any error, returning 0".to_string(),
                }
            },
            Err(e) => {
                //let retcode = e.get_retcode();
                response = StartContainerResponse {
                    code: "Error on exit: \n\t{} \n\tReturning {retcode not found}".to_string(),
                }
            }
        }

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

    env_logger::Builder::from_default_env()
        .format_timestamp_secs()
        .filter(None, log::LevelFilter::Debug)
        .init();

    Server::builder()
        .add_service(reflection_service)
        .add_service(RustualizeServer::new(rustualize))
        .serve(addr)
        .await?;

    Ok(())
}
