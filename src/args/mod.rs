use clap::Clap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clap, Default, Serialize, Deserialize)]
#[clap(
    version = "0.1.1",
    author = "Mahdi Robatipoor <mahdi.robatipoor@gmail.com>"
)]
pub struct Args {
    #[clap(short, long)]
    share_secret_key: String,
    #[clap(short, long)]
    ip_addr: String,
    #[clap(short, long)]
    ipv4_addr: String,
    #[clap(short, long)]
    ipv6_addr: String,
    #[clap(short, long)]
    port_numbers: Vec<u32>,
    #[clap(short, long)]
    password: String,
    #[clap(short, long)]
    is_sender: bool,
    #[clap(short, long)]
    debug_flag: bool,
    #[clap(short, long)]
    no_prompt: bool,
    #[clap(short, long)]
    only_local: bool,
    #[clap(short, long)]
    disable_local: bool,
    #[clap(short, long)]
    no_compress: bool,
    #[clap(short, long)]
    no_multiplexing: bool,
    #[clap(short, long)]
    std_out: bool,
    #[clap(short, long)]
    ignore_stdin: bool,
    #[clap(short, long)]
    sending_text: bool,
    #[clap(short, long)]
    ask: bool,
}

pub fn get_args() -> Args {
    Args::parse()
}
