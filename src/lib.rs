use std::{
    net::TcpListener,
    ops::Add,
    time::{Duration, SystemTime},
};

use actix_web::{
    dev::{Server, ServiceRequest},
    middleware::Logger,
    web::{self, Data, ReqData},
    App, Error, HttpMessage, HttpResponse, HttpServer, Responder,
};
use actix_web_httpauth::{
    extractors::{
        bearer::{BearerAuth, Config},
        AuthenticationError,
    },
    middleware::HttpAuthentication,
};
use biscuit_auth::{
    builder::{Fact, Term},
    Biscuit, KeyPair,
};

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

async fn get_valid_token(root: Data<BiscuitSec>, name: web::Path<String>) -> impl Responder {
    let mut builder = Biscuit::builder(&root.keypair);
    builder
        .add_authority_fact(Fact::new(
            "user".to_string(),
            vec![Term::Str(name.to_string())],
        ))
        .unwrap();
    builder
        .add_authority_fact(Fact::new(
            "expiration".to_string(),
            vec![Term::Date(
                SystemTime::now()
                    .add(Duration::from_secs(600))
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            )],
        ))
        .unwrap();

    builder
        .add_authority_check(
            r#"check if time($time), expiration($expiration), $time < $expiration"#,
        )
        .unwrap();

    let biscuit = builder.build().unwrap();
    biscuit.to_base64().unwrap()
}

async fn hello(msg: ReqData<User>) -> impl Responder {
    format!("hello {:?}", *msg)
}

#[derive(Clone, Debug)]
struct User(String);

async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    let root = req.app_data::<Data<BiscuitSec>>().unwrap();
    let biscuit = Biscuit::from_base64(credentials.token(), |_| root.keypair.public())
        .map_err(|_| AuthenticationError::from(Config::default()))?;

    let user = authorize(&biscuit).map_err(|_| AuthenticationError::from(Config::default()))?;

    req.extensions_mut().insert(User(user.to_owned()));
    Ok(req)
}

fn authorize(token: &Biscuit) -> Result<String, ()> {
    let mut authorizer = token.authorizer().map_err(|_| ())?;

    authorizer.set_time();
    authorizer.allow().map_err(|_| ())?;
    authorizer.authorize().map_err(|_| ())?;

    let res: Vec<(String,)> = authorizer
        .query("data($name) <- user($name)")
        .map_err(|_| ())?;
    Ok(res.get(0).ok_or(())?.0.clone())
}

#[derive(Clone)]
struct AndWrapper(BiscuitSec);

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
            .route("/health_check", web::get().to(health_check))
            .route("/get_valid_token/{name}", web::get().to(get_valid_token))
            .service(
                web::scope("/api")
                    .app_data(Data::clone(&root))
                    .wrap(HttpAuthentication::bearer(validator))
                    .route("/hello", web::get().to(hello)),
            )
    })
    .listen(listener)?
    .run();
    Ok(server)
}
