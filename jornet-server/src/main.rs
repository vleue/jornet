use std::net::TcpListener;

use jornet_server::{configuration::get_configuration, run};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = dbg!(get_configuration());

    let address = format!("{}:{}", "127.0.0.1", configuration.application_port);
    let listener = TcpListener::bind(&address)?;

    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    run(listener, connection_pool)?.await
}
