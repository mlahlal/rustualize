use crate::errors::Errcode;

use std::fs::{canonicalize, remove_dir};
use std::convert::TryInto;
use nix::unistd::Pid;
use rlimit::{setrlimit, Resource};
use cgroups_rs::{cgroup_builder::CgroupBuilder, MaxValue};
use cgroups_rs::CgroupPid;
use cgroups_rs::hierarchies::V2;

const KMEM_LIMIT: i64 = 2048 * 1024 * 1024;
const MEM_LIMIT: i64 = KMEM_LIMIT;
const MAX_PID: MaxValue = MaxValue::Value(256);
const NOFILE_RLIMIT: u64 = 1024;

pub fn restrict_resources(hostname: &String, pid: Pid) -> Result<(), Errcode> {
    log::debug!("Restricting resources for hostname {}", hostname);

    let Ok(cgs) = CgroupBuilder::new(hostname)
        .cpu().shares(256).done()
        //.memory().kernel_memory_limit(KMEM_LIMIT).memory_hard_limit(MEM_LIMIT).done()
        .memory().memory_hard_limit(MEM_LIMIT).done()
        .pid().maximum_number_of_processes(MAX_PID).done()
        .blkio().weight(50).done()
        .build(Box::new(V2::new()))
    else {
        return Err(Errcode::ResourcesError(0));
    };

    let pid: u64 = pid.as_raw().try_into().unwrap();

    if let Err(e) = cgs.add_task_by_tgid(CgroupPid::from(pid)) {
        log::error!("error: {}", e);
        return Err(Errcode::ResourcesError(1));
    }

    if let Err(_) = setrlimit(Resource::NOFILE, NOFILE_RLIMIT, NOFILE_RLIMIT) {
        return Err(Errcode::ResourcesError(2));
    }

    Ok(())
}

pub fn clean_cgroups(hostname: &String) -> Result<(), Errcode> {
    log::debug!("Cleaning cgroups");

    match canonicalize(format!("/sys/fs/cgroup/{}/", hostname)) {
        Ok(d) => {
            if let Err(_) = remove_dir(d) {
                return Err(Errcode::ResourcesError(3));
            }
        },
        Err(e) => {
            log::error!("Error while canonicalize path: {}", e);
            return Err(Errcode::ResourcesError(4));
        }
    }

    Ok(())
}
