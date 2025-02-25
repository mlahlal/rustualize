use crate::errors::Errcode;

use std::path::PathBuf;
use gumdrop::Options;

#[derive(Debug, Options)]
pub struct MyOptions {
    #[options(help = "print help message")]
    help: bool,

    #[options(help = "activate debug mode")]
    debug: bool,

    #[options(help = "command to execute inside the container", required)]
    pub command: String,

    #[options(help = "user id to create inside container")]
    pub uid: u32,

    #[options(help = "directory to mount as root of container", long = "mount")]
    pub mount_dir: PathBuf,
}

pub fn parse_args() -> Result<MyOptions, Errcode> {
    let opts = MyOptions::parse_args_default_or_exit();

    if opts.debug{
        setup_log(log::LevelFilter::Debug);
    } else {
        setup_log(log::LevelFilter::Info);
    }

    if opts.command.is_empty() {
        return Err(Errcode::ArgumentInvalid("command"));
    }

    if !opts.mount_dir.exists() || !opts.mount_dir.is_dir() {
        return Err(Errcode::ArgumentInvalid("mount"));
    }

    Ok(opts)
}

pub fn setup_log(level: log::LevelFilter) {
    env_logger::Builder::from_default_env()
        .format_timestamp_secs()
        .filter(None, level)
        .init();
}
