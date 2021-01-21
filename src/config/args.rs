use clap::Clap;

#[derive(Debug, Clap)]
#[clap(
    version = "0.0.1",
    author = "mahdi robatipoor <mahdi.robatipoor@gmail.com>"
)]
pub struct Args {}

pub fn get_args() -> Args {
    Args::parse()
}
