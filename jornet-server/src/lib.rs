use std::net::TcpListener;

use actix_web::{
    dev::Server,
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use configuration::get_configuration;
use sqlx::PgPool;

pub mod configuration;
pub mod domains;

pub fn run(listener: TcpListener, connection_pool: PgPool) -> Result<Server, std::io::Error> {
    let config = get_configuration();
    let root = Data::new(config.get_keypair());
    let connection = Data::new(connection_pool);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(connection.clone())
            .app_data(root.clone())
            .wrap(Logger::default())
            .route(
                "/health_check",
                web::get().to(domains::healthcheck::health_check),
            )
            .service(domains::admins::admins(root.clone()))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
