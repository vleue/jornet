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
pub mod domains;

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
                "/test_bed/get_valid_token/{name}",
                web::get().to(domains::test_bed::get_valid_token),
            )
            .service(
                web::scope("/test_bed")
                    .app_data(Data::clone(&root))
                    .wrap(HttpAuthentication::bearer(domains::test_bed::validator))
                    .route("/hello", web::get().to(domains::test_bed::hello))
                    .route("/goodbye", web::delete().to(domains::test_bed::goodbye)),
            )
    })
    .listen(listener)?
    .run();
    Ok(server)
}
