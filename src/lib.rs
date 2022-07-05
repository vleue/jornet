use std::net::TcpListener;

use actix_web::{
    dev::Server,
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use actix_web_httpauth::middleware::HttpAuthentication;
use biscuit_auth::KeyPair;
use sqlx::PgPool;

pub mod configuration;
mod domains;

pub fn run(listener: TcpListener, connection_pool: PgPool) -> Result<Server, std::io::Error> {
    let root = Data::new(KeyPair::new());
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
            .route(
                "/get_valid_token/{name}",
                web::get().to(domains::admin::get_valid_token),
            )
            .service(
                web::scope("/api")
                    .app_data(Data::clone(&root))
                    .wrap(HttpAuthentication::bearer(domains::admin::validator))
                    .route("/hello", web::get().to(domains::admin::hello))
                    .route("/goodbye", web::delete().to(domains::admin::goodbye)),
            )
    })
    .listen(listener)?
    .run();
    Ok(server)
}
