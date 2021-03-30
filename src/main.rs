use args::get_args;
use rk::server::Server;

mod args;

#[tokio::main]
async fn main() {
    let _args = get_args();
    let server = Server::new("banner", "password");
    server.run("127.0.0.1:8080").await.unwrap();
    println!("rock cli tool");
}
