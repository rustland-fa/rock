use pnet::ipnetwork::IpNetworkError;
use tokio_tungstenite::tungstenite;
use trust_dns_resolver::error::ResolveError;
// TODO COMPELTE ERROR
#[derive(Debug)]
pub enum Error {
    ResponseError(String),
    SerializeJsonError(String),
    Other(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for Error {}

impl From<String> for Error {
    fn from(msg: String) -> Self {
        Self::Other(msg)
    }
}

impl From<&str> for Error {
    fn from(msg: &str) -> Self {
        Self::Other(msg.to_string())
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(msg: serde_json::error::Error) -> Self {
        Self::Other(msg.to_string())
    }
}

impl From<IpNetworkError> for Error {
    fn from(msg: IpNetworkError) -> Self {
        Self::Other(msg.to_string())
    }
}

impl From<ResolveError> for Error {
    fn from(msg: ResolveError) -> Self {
        Self::Other(msg.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(msg: std::io::Error) -> Self {
        Self::Other(msg.to_string())
    }
}

impl From<reqwest::Error> for Error {
    fn from(msg: reqwest::Error) -> Self {
        Self::Other(msg.to_string())
    }
}

impl From<std::net::AddrParseError> for Error {
    fn from(msg: std::net::AddrParseError) -> Self {
        Self::Other(msg.to_string())
    }
}

impl From<tokio::sync::mpsc::error::SendError<tungstenite::Message>> for Error {
    fn from(msg: tokio::sync::mpsc::error::SendError<tungstenite::Message>) -> Self {
        Self::Other(msg.to_string())
    }
}

impl From<tungstenite::error::Error> for Error {
    fn from(msg: tungstenite::error::Error) -> Self {
        Self::Other(msg.to_string())
    }
}
