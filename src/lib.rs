#![allow(dead_code)]

pub type Result<T = ()> = std::result::Result<T, crate::error::Error>;

pub mod args;
pub mod client;
pub mod compress;
pub mod config;
pub mod constants;
pub mod crypt;
pub mod error;
pub mod message;
pub mod progress;
pub mod server;
pub mod utils;
