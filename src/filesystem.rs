use crate::errors::Errcode;

use flate2::read::GzDecoder;
use std::io::{BufReader, Write, copy};
use std::fs::{File, create_dir_all};
use std::path::PathBuf;
use reqwest::blocking::get;
use tar::Archive;
use nix::mount::{mount, MsFlags};
use std::ffi::CString;

pub fn setfilesystem() -> Result<(), Errcode> {
    get_rootfs("https://ftp.debian.org/debian/dists/stable/main/installer-amd64/current/images/netboot/netboot.tar.gz", "/home/mouad/Documents/dev/rustualize/containerdisk/")?;
    setconfigfiles("/home/mouad/Documents/dev/rustualize/containerdisk/")?;
    mountfilesystem("/home/mouad/Documents/dev/rustualize/containerdisk/")?;

    Ok(())
}

fn get_rootfs(url: &str, dest: &str) -> Result<(), Errcode> {
    log::debug!("Downloading rootfs...");

    let mut res = get(url).expect("non funsiona 1");
    let mut archive_file = File::create("/tmp/rootfs.tar.gz").expect("non funsiona 3");
    copy(&mut res, &mut archive_file);

    log::debug!("Extracting root filesystem in {}...", dest);

    let targz = File::open("/tmp/rootfs.tar.gz");
    let decompressor = GzDecoder::new(BufReader::new(targz.unwrap()));
    let mut archive = Archive::new(decompressor);
    if let Err(_) = archive.unpack(dest) {
        return Err(Errcode::ArgumentInvalid("archive unpack andato in vacca"));
    }

    log::debug!("Successfully extracted rootfs in {}", dest);

    Ok(())
}

fn setconfigfiles(rootfs: &str) -> Result<(), Errcode> {
    let etc_path = format!("{}/etc", rootfs);

    if let Err(e) = create_dir_all(&etc_path) {
        return Err(Errcode::ArgumentInvalid("errore create dir all"));
    }

    let passwd_content = "root:x:0:0:root:/root:/bin/sh\n";
    let mut passwd = match File::create(format!("{}/passwd", &etc_path)) {
        Ok(file) => file,
        Err(e) => return Err(Errcode::ArgumentInvalid("passwd file")),
    };

    if let Err(e) = passwd.write_all(passwd_content.as_bytes()) {
        return Err(Errcode::ArgumentInvalid("passwd file write"));
    }

    let group_content = "root:x:0:\n";
    let mut group = match File::create(format!("{}/group", &etc_path)) {
        Ok(file) => file,
        Err(e) => return Err(Errcode::ArgumentInvalid("group file")),
    };

    if let Err(e) = group.write_all(group_content.as_bytes()) {
        return Err(Errcode::ArgumentInvalid("group file write"));
    }

    let resolv_content = "nameserver 1.1.1.1\n";
    let mut resolv = match File::create(format!("{}/resolv.conf", &etc_path)) {
        Ok(file) => file,
        Err(e) => return Err(Errcode::ArgumentInvalid("resolv file")),
    };

    if let Err(e) = resolv.write_all(resolv_content.as_bytes()) {
        return Err(Errcode::ArgumentInvalid("resolv file write"));
    }

    log::debug!("set config files");

    Ok(())
}

fn mountfilesystem(rootfs: &str) -> Result<(), Errcode> {
    let mount_points = [
        ("proc", format!("{}/proc", rootfs), "proc"),
        ("sysfs", format!("{}/sys", rootfs), "sysfs"),
        ("tmpfs", format!("{}/dev", rootfs), "tmpfs"),
    ];

    for (source, target, fstype) in &mount_points {
        if let Err(e) = create_dir_all(target) {
            return Err(Errcode::ArgumentInvalid("create dir all mounting system"));
        }
        /*let src_c = CString::new(*source).unwrap();
        let target_c = CString::new(target.as_str()).unwrap();
        let fstype_c = CString::new(*fstype).unwrap();*/


        let src_c = PathBuf::from(*source);
        let target_c = PathBuf::from(target.as_str());
        let fstype_c = PathBuf::from(*fstype);

        if let Err(e) = unsafe {
            mount(
                Some(&src_c),
                &target_c,
                Some(&fstype_c),
                MsFlags::empty(),
                None::<&str>,
            )
        } {
            return Err(Errcode::ArgumentInvalid("unsafe code doing unsafe things"));
        }
    }

    Ok(())
}
