use errors::exit_with_retcode;
use std::process::exit;

mod cli;
mod errors;
mod config;
mod container;
mod ipc;
mod child;
mod hostname;
mod mounts;
mod namespaces;
mod capabilities;
mod syscalls;
mod resources;
mod filesystem;

fn main() -> std::io::Result<()> {
    match cli::parse_args() {
        Ok(args) => {
            log::info!("{:?}", args);
            exit_with_retcode(container::start(args));
        },
        Err(e) => {
            log::error!("Error while parsing arguments: \n\t{}", e);
            exit(e.get_retcode());
        }
    }

    if true {
        return Ok(());
    }

    Ok(())
}
