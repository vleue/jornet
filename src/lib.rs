use std::net::TcpListener;

use actix_web::{
    dev::Server,
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use actix_web_httpauth::middleware::HttpAuthentication;
use biscuit_auth::KeyPair;

mod domains;

struct BiscuitSec {
    keypair: KeyPair,
}

impl BiscuitSec {
    fn from(kp: KeyPair) -> BiscuitSec {
        Self { keypair: kp }
    }
}

impl Clone for BiscuitSec {
    fn clone(&self) -> Self {
        Self {
            keypair: KeyPair::from(self.keypair.private().clone()),
        }
    }
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let root = Data::new(BiscuitSec::from(KeyPair::new()));

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&root))
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
                    .route("/hello", web::get().to(domains::admin::hello)),
            )
    })
    .listen(listener)?
    .run();
    Ok(server)
}
