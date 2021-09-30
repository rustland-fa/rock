#![allow(dead_code)]

use std::result;

pub type Result<T = String> = result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

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
