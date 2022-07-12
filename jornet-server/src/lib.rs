use std::{net::TcpListener, path::PathBuf, str::FromStr};

use actix_files::NamedFile;
use actix_web::{
    dev::Server,
    middleware::Logger,
    web::{self, Data},
    App, HttpRequest, HttpServer, Result,
};
use configuration::get_configuration;
use sqlx::PgPool;

pub mod configuration;
pub mod domains;

async fn spa(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    let mut static_path = PathBuf::from_str("static").unwrap();
    if path.extension().is_some() {
        static_path.push(path);
    } else {
        static_path.push("index.html");
    }
    Ok(NamedFile::open(static_path)?)
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
            .route(
                "/health_check",
                web::get().to(domains::healthcheck::health_check),
            )
            .service(domains::config::config(config.clone()))
            .service(domains::oauth::oauth())
            .service(domains::admins::admins(root.clone()))
            .route("/{filename:.*}", web::get().to(spa))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
