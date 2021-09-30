use crate::{
    config::{IpVersion, NONCE_LENGTH},
    constants::word::WORDS,
};
use pnet::datalink;
use rand::{seq::SliceRandom, thread_rng, Rng};
use serde::{de::DeserializeOwned, Serialize};
use sha2::{Digest, Sha256};
use std::net::IpAddr;
use tokio::net::UdpSocket;
use trust_dns_resolver::{config::*, Resolver};

pub fn generate_random_bytes() -> [u8; NONCE_LENGTH] {
    thread_rng().gen::<[u8; NONCE_LENGTH]>()
}

pub fn json_bytes_to_data<T: DeserializeOwned>(slice: &[u8]) -> crate::Result<T> {
    let data = serde_json::from_slice(slice)?;
    Ok(data)
}

pub fn from_json<T: DeserializeOwned>(json: &str) -> crate::Result<T> {
    serde_json::from_str(json).map_err(|e| e.into())
}

pub fn to_json<T: ?Sized + Serialize>(value: &T) -> crate::Result<String> {
    serde_json::to_string(value).map_err(|e| e.into())
}

pub fn sha256(data: &[u8]) -> crate::Result<String> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    byte_to_hex_lower_case(&hasher.finalize()[..])
}

pub fn byte_to_hex_lower_case(data: &[u8]) -> crate::Result<String> {
    Ok(data
        .iter()
        .map(|b| format!("{:02x?}", b))
        .collect::<String>())
}

pub fn byte_to_hex_upper_case(data: &[u8]) -> crate::Result<String> {
    Ok(data
        .iter()
        .map(|b| format!("{:02X?}", b))
        .collect::<String>())
}

pub async fn public_ip_addr(ip_version: IpVersion) -> crate::Result<IpAddr> {
    let resp = match ip_version {
        IpVersion::V4 => get("https://ipv4.icanhazip.com").await?,
        IpVersion::V6 => get("https://icanhazip.com").await?,
    };
    let ip = resp.trim().parse()?;
    Ok(ip)
}

pub async fn get(url: &str) -> crate::Result<String> {
    reqwest::get(url).await?.text().await.map_err(|e| e.into())
}

pub async fn local_ip_v4_addr() -> crate::Result<IpAddr> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    socket.connect("8.8.8.8:80").await?;
    let addr = socket.local_addr()?;
    Ok(addr.ip())
}

pub fn local_ip_addr(ip_version: IpVersion) -> crate::Result<Vec<IpAddr>> {
    let ips = datalink::interfaces()
        .iter()
        .filter(|i| i.is_up() && !i.is_loopback())
        .map(|i| i.ips.clone())
        .flatten()
        .filter(|i| match ip_version {
            IpVersion::V4 => i.is_ipv4(),
            IpVersion::V6 => i.is_ipv6(),
        })
        .map(|i| i.ip())
        .collect::<Vec<IpAddr>>();
    Ok(ips)
}

pub fn lookup_id(addr: &str) -> crate::Result<Vec<IpAddr>> {
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default())?;
    let response = resolver.lookup_ip(addr)?;
    Ok(response.iter().collect())
}

pub fn get_rand_word(amount: usize) -> String {
    let mut rng = rand::thread_rng();
    WORDS
        .choose_multiple(&mut rng, amount)
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join("-")
}
