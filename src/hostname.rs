use crate::errors::Errcode;

use rand::Rng;
use rand::seq::SliceRandom;
use nix::unistd::sethostname;

const HOSTNAME_NAMES: [&'static str; 8] = [
    "centopercento", "poppiesfromsrilanka", "filmsenzavolume", "(arrivo?)",
    "stupidi", "bolobynight", "ghettoblaster", "disneyinferno"
];

const HOSTNAME_ADJ: [&'static str; 16] = [
    "kaos", "fibra", "nerone", "bassi",
    "salmo", "nitro", "jthesmoker", "kidyugi",
    "inoki", "cgold", "8blevrai", "egreen",
    "nayt", "jm", "gue", "rancore"
];

pub fn generate_hostname() -> Result<String, Errcode> {
    let mut rng = rand::thread_rng();
    let num = rng.gen::<u8>();

    let name = HOSTNAME_NAMES.choose(&mut rng).ok_or(Errcode::RngError)?;
    let adj = HOSTNAME_ADJ.choose(&mut rng).ok_or(Errcode::RngError)?;

    Ok(format!("{}-{}-{}", adj, name, num))
}

pub fn set_container_hostname (hostname: &String) -> Result<(), Errcode> {
    match sethostname(hostname) {
        Ok(_) => {
            log::debug!("Container hostname is now {}", hostname);
            Ok(())
        },
        Err(_) => {
            log::error!("Cannot set hostname {} for container", hostname);
            Err(Errcode::HostnameError(0))
        }
    }
}
