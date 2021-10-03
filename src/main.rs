use args::get_args;
use rk::server::Server;

mod args;

#[tokio::main]
async fn main() {
    let _args = get_args();
    let server = Server::new(
        "127.0.0.1:8080".to_string(),
        "username".to_string(),
        "password".to_string(),
    );
    server.run().await.unwrap();
}
