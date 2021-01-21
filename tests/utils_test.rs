use rock::config;
use rock::utils;

#[test]
fn sha256_test() {
    let hash = utils::sha256(b"Hello World!").unwrap();
    println!("sha256 Hello World! is => {:?}", hash);
    assert_eq!(
        "7f83b1657ff1fc53b92dc18148a1d65dfc2d4b1fa3d677284addd200126d9069",
        hash
    )
}

#[tokio::test]
async fn async_public_ip_v4_addr_test() {
    let addr = utils::public_ip_addr(config::IpVersion::V4).await.unwrap();
    println!("public ip addr v4 => {:?}", addr);
    assert!(addr.is_ipv4())
}

#[tokio::test]
async fn async_public_ip_v6_addr_test() {
    let addr = utils::public_ip_addr(config::IpVersion::V6).await.unwrap();
    println!("public ip addr v6 => {:?}", addr);
    assert!(addr.is_ipv6())
}

#[tokio::test]
async fn async_local_ip_v4_test() {
    let addr = utils::local_ip_v4_addr().await.unwrap();
    println!("local ip => {:?}", addr);
    assert!(addr.is_ipv4())
}

#[test]
fn lookup_ip_v4_test() {
    let response = utils::lookup_id(config::DEFAULT_RELAY_ADDR_V6).unwrap();
    println!("lookup ip v4 test response => {:?}", response);
    assert!(response.len() >= 1);
}

#[test]
fn lookup_ip_v6_test() {
    let response = utils::lookup_id(config::DEFAULT_RELAY_ADDR_V6).unwrap();
    println!("lookup ip v6 test response => {:?}", response);
    assert!(response.len() >= 1);
}

#[test]
fn get_rand_word_test() {
    let words = utils::get_rand_word(3);
    println!("words => {}", words);
    assert!(words.len() > 0 && words.split("-").count() == 3);
}

#[test]
fn local_ip_v4_addr_test() {
    let v4 = crate::config::IpVersion::V4;
    let addr = utils::local_ip_addr(v4).unwrap();
    println!("local ip v4 => {:?}", addr);
    assert!(addr.len() > 0)
}

#[test]
fn local_ip_v6_addr_test() {
    let v6 = crate::config::IpVersion::V6;
    let addr = utils::local_ip_addr(v6).unwrap();
    println!("local ip v6 => {:?}", addr);
    assert!(addr.len() > 0)
}
