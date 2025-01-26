use crate::cli::MyOptions;
use crate::errors::Errcode;
use crate::config::ContainerOpts;
use crate::child::generate_child_process;
use crate::mounts::clean_mounts;

use std::os::fd::RawFd;
use nix::unistd::Pid;
use nix::sys::wait::{self, waitpid};

pub struct Container {
    config: ContainerOpts,
    sockets: (RawFd, RawFd),
    child_pid: Option<Pid>,
}

impl Container {
    pub fn new(args: MyOptions) -> Result<Container, Errcode> {
        let (config, sockets) = ContainerOpts::new(
            args.command,
            args.uid,
            args.mount_dir,
        )?;

        Ok(
            Container {
                config,
                sockets,
                child_pid: None,
            }
        )
    }

    pub fn create(&mut self) -> Result<(), Errcode> {
        let pid = generate_child_process(self.config.clone())?;
        self.child_pid = Some(pid);
        log::debug!("Creation finished");
        Ok(())
    }

    pub fn clean_exit(&mut self) -> Result<(), Errcode> {
        log::debug!("Cleaning containers");

        clean_mounts(&self.config.mount_dir)?;

        Ok(())
    }
}

pub fn wait_child (pid: Option<Pid>) -> Result<(), Errcode> {
    if let Some(child_pid) = pid {
        log::debug!("Waiting child (pid: {}) to finish", child_pid);

        if let Err(e) = waitpid(child_pid, None) {
            log::error!("Error while waiting for pid to finish {:?}", e);
            return Err(Errcode::ContainerError(1));
        }
    }

    Ok(())
}

pub fn start(args: MyOptions) -> Result<(), Errcode> {
    let mut container = Container::new(args)?;

    log::debug!("Container sockets : ({}, {})", container.sockets.0, container.sockets.1);

    if let Err(e) = container.create() {
        container.clean_exit()?;
        log::error!("Error while creating container {:?}", e);
        return Err(e);
    }

    log::debug!("Container child PID : {:?}", container.child_pid);

    wait_child(container.child_pid)?;

    log::debug!("Finished, cleaning & exit");

    container.clean_exit()
}
