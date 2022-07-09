use std::net::TcpListener;

use actix_web::{
    dev::Server,
    middleware::Logger,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder, Result,
};
use configuration::get_configuration;
use sqlx::PgPool;

pub mod configuration;
pub mod domains;

async fn index() -> impl Responder {
    HttpResponse::Ok().content_type("Text/Html").body(
        r#"
<html>

<head>
    <title>Jornet</title>
</head>

<body><a href="admin">Connect</a>
</body>

</html>
    "#,
    )
}

pub fn run(listener: TcpListener, connection_pool: PgPool) -> Result<Server, std::io::Error> {
    let config = Data::new(get_configuration());
    let root = Data::new(config.get_keypair());
    let connection = Data::new(connection_pool);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(connection.clone())
            .app_data(root.clone())
            .app_data(config.clone())
            .wrap(Logger::default())
            .route("/", web::get().to(index))
            .route(
                "/health_check",
                web::get().to(domains::healthcheck::health_check),
            )
            .service(domains::admin_site::admin_site())
            .service(domains::admins::admins(root.clone()))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
