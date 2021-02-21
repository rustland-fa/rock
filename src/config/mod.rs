use crate::args::Args;
use crate::utils;
use once_cell::sync::Lazy;
use std::{
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    path::PathBuf,
};

pub const BUFFER_SIZE: usize = 1024 * 64;
pub const NONCE_LENGTH: usize = 12;
pub const DEFAULT_RELAY_ADDR_V4: &str = "croc.schollz.com";
pub const DEFAULT_RELAY_ADDR_V6: &str = "croc6.schollz.com";
pub const DEFAULT_POTR: u32 = 1000;
pub const DEFAULT_PASSPHRASE: &str = "123abc";

pub enum IpVersion {
    V4,
    V6,
}

pub static RELAY_IP_ADDR_V6: Lazy<IpAddr> = Lazy::new(|| {
    utils::lookup_id(DEFAULT_RELAY_ADDR_V6)
        .unwrap_or_else(|_e| vec![IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0))])
        .into_iter()
        .next()
        .unwrap()
});
pub static RELAY_IP_ADDR: Lazy<IpAddr> = Lazy::new(|| {
    utils::lookup_id(DEFAULT_RELAY_ADDR_V4)
        .unwrap_or_else(|_e| vec![IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))])
        .into_iter()
        .next()
        .unwrap()
});

pub static CONFIG_ARGS: Lazy<Args> =
    Lazy::new(|| confy::load(&*get_config_file().unwrap()).unwrap_or_default());

pub fn get_config_file() -> Option<String> {
    get_home_dir_config()
        .and_then(|p| {
            Some(
                p.join("config.toml")
                    .to_str()
                    .to_owned()
                    .and_then(|p| Some(p.to_string())),
            )
        })
        .flatten()
}

pub fn get_home_dir_config() -> Option<PathBuf> {
    dirs::home_dir().and_then(|p| Some(p.join(".config").join("rock")))
}
