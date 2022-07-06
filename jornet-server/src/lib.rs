use std::net::TcpListener;

use actix_web::{
    dev::Server,
    middleware::Logger,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder, Result,
};
use configuration::{get_configuration, Settings};
use sqlx::PgPool;

pub mod configuration;
pub mod domains;

async fn index(config: web::Data<Settings>) -> impl Responder {
    HttpResponse::Ok().content_type("Text/Html").body(format!(
        r#"
<html>

<head>
    <title>Jornet Admin Panel</title>
</head>

<body><a href="https://github.com/login/oauth/authorize?client_id={}">Authenticate with GitHub</a>
</body>

</html>
    "#,
        config.github_admin_app.client_id
    ))
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
            .service(domains::admins::admins(root.clone()))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
