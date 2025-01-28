use errors::exit_with_retcode;
use nix::sched::{unshare, CloneFlags};
use nix::unistd::{fork, ForkResult, write};
use std::os::unix::fs::chroot;
use std::process::Command;
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

    let flags = CloneFlags::CLONE_NEWPID
        | CloneFlags::CLONE_NEWIPC
        | CloneFlags::CLONE_NEWUTS
        | CloneFlags::CLONE_NEWNET
        | CloneFlags::CLONE_NEWUSER;

    unshare(flags)
        .expect("Failed to unshare PID namespace");

    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            chroot("/rustualize")
                .expect("Failed to chroot to /rustualize");
            std::env::set_current_dir("/")
                .expect("Failed to set current directory to /");

            write(std::io::stdout(), "Child process started\n".as_bytes()).ok();

            Command::new("sh")
                .arg("-c")
                .arg("echo ciao >> home/tmp.txt")
                .status()
                .expect("failed command");
        }
        Ok(ForkResult::Parent { child }) => {
            println!("Parent process (child PID: {})", child);
        }
        Err(err) => {
            eprintln!("Fork failed: {}", err);
        }
    }

    Ok(())
}
