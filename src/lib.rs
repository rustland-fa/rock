#![allow(dead_code)]

use std::{error, result};

pub type Result<T = String> = result::Result<T, Box<dyn error::Error + Send + Sync + 'static>>;

pub mod message;

pub mod compress;

pub mod crypt;

pub mod utils;

pub mod config;

pub mod constants;

pub mod connection;

pub mod server;
