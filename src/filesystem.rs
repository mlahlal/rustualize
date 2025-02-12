use crate::errors::Errcode;
use crate::config::ContainerOpts;

use std::process::Command;

pub fn setfilesystem(config: ContainerOpts) -> Result<(), Errcode> {
    debootstrap(&config.mount_dir.to_str().unwrap())?;

    Ok(())
}

fn debootstrap(dest: &str) -> Result<(), Errcode> {
    log::debug!("Executing debootstrap");

    match Command::new("sudo")
        .arg("debootstrap")
        .arg("stable")
        .arg(dest)
        .arg("http://deb.debian.org/debian")
        .output() {
            Ok(output) => log::debug!("debootstrap output : {}", output),
            Err(e) => {
                log::error!("Debootstrap error : {:?}", e);
                return Err(Errcode::ArgumentInvalid("debootstrap error"));
            },
    }

    Ok(())
}
