use std::net::TcpListener;

use jornet_server::{configuration::get_configuration, run};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration();

    let address = format!(
        "{}:{}",
        configuration.application_host, configuration.application_port
    );
    let listener = TcpListener::bind(&address)?;

    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    run(listener, connection_pool)?.await
}
