use std::net::TcpListener;

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
    Authorizer, Biscuit, KeyPair,
};
use chrono::{Duration, Utc};

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

async fn get_valid_token(root: Data<BiscuitSec>, name: web::Path<String>) -> impl Responder {
    let mut builder = Biscuit::builder(&root.keypair);
    builder
        .add_authority_fact(User(name.to_string()).as_biscuit_fact())
        .unwrap();

    builder
        .add_authority_check(
            format!(
                r#"check if time($time), $time < {}"#,
                dbg!((Utc::now() + Duration::seconds(600)).to_rfc3339())
            )
            .as_str(),
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

impl User {
    fn as_biscuit_fact(&self) -> Fact {
        Fact::new("user".to_string(), vec![Term::Str(self.0.to_string())])
    }

    fn from_biscuit(biscuit: &Biscuit) -> Result<Self, ()> {
        let mut authorizer = biscuit.authorizer().map_err(|_| ())?;
        Self::from_authorizer(&mut authorizer)
    }

    fn from_authorizer(authorizer: &mut Authorizer) -> Result<Self, ()> {
        let res: Vec<(String,)> = authorizer
            .query("data($name) <- user($name)")
            .map_err(|_| ())?;
        Ok(User(res.get(0).ok_or(())?.0.clone()))
    }
}

async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    let root = req.app_data::<Data<BiscuitSec>>().unwrap();
    let biscuit = Biscuit::from_base64(credentials.token(), |_| root.keypair.public())
        .map_err(|_| AuthenticationError::from(Config::default()))?;

    let user = authorize(&biscuit).map_err(|_| AuthenticationError::from(Config::default()))?;

    req.extensions_mut().insert(user);
    Ok(req)
}

fn authorize(token: &Biscuit) -> Result<User, ()> {
    let mut authorizer = token.authorizer().map_err(|_| ())?;

    authorizer.set_time();
    dbg!(authorizer.allow()).map_err(|_| ())?;
    dbg!(authorizer.authorize()).map_err(|_| ())?;

    User::from_authorizer(&mut authorizer)
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
