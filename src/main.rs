use nix::sched::{unshare, CloneFlags};
use nix::unistd::{fork, ForkResult, write};
use std::os::unix::fs::chroot;
use std::process::Command;

fn main() -> std::io::Result<()> {
    unshare(CloneFlags::CLONE_NEWPID).expect("Failed to unshare PID namespace");

    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            chroot("/rustualize").expect("Failed to chroot to /rustualize");
            std::env::set_current_dir("/").expect("Failed to set current directory to /");

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
