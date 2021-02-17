use rock::server::Server;

mod args;

use args::get_args;

#[tokio::main]
async fn main() {
    let _args = get_args();
    let server = Server::new("banner", "password");
    server.run("127.0.0.1:8080").await.unwrap();
    println!("rock cli tool");
}
