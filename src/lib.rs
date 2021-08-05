#![allow(dead_code)]

use std::{error, result};

pub type Result<T = String> = result::Result<T, Box<dyn error::Error + Send + Sync + 'static>>;

pub mod args;
pub mod client;
pub mod codec;
pub mod compress;
pub mod config;
pub mod connection;
pub mod constants;
pub mod crypt;
pub mod frame;
pub mod message;
pub mod progress;
pub mod server;
pub mod utils;
