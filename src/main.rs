use std::net::TcpListener;

use jornet::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");

    let address = format!("{}:{}", "127.0.0.1", 8080);
    let listener = TcpListener::bind(&address)?;
    run(listener)?.await
}
