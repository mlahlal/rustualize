use std::fmt;
use std::process::exit;

#[derive(Debug)]
pub enum Errcode {
    ArgumentInvalid(&'static str),
    ContainerError(u8),
    SocketError(u8),
    ChildProcessError(u8),
    HostnameError(u8),
    RngError,
    MountsError(u8),
    NamespaceError(u8),
    CapabilitiesError(u8),
    SyscallsError(u8),
    ResourcesError(u8),
}

#[allow(unreachable_patterns)]
impl fmt::Display for Errcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Errcode::ArgumentInvalid(element) => write!(f, "ArgumentInvalid: {}", element),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl Errcode {
    pub fn get_retcode(&self) -> i32 {
        1
    }
}

pub fn exit_with_retcode(res: Result<(), Errcode>) {
    match res {
        Ok(_) => {
            log::debug!("Exit without any error, returning 0");
            exit(0);
        },
        Err(e) => {
            let retcode = e.get_retcode();
            log::error!("Error on exit: \n\t{} \n\tReturning {}", e, retcode);
            exit(retcode);
        }
    }
}
