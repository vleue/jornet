use actix_web::{
    dev::ServiceRequest,
    web::{self, Data, ReqData},
    Error, HttpMessage, Responder,
};
use actix_web_httpauth::extractors::{
    bearer::{BearerAuth, Config},
    AuthenticationError,
};
use biscuit_auth::{
    builder::{Fact, Term},
    Authorizer, Biscuit,
};
use chrono::{Duration, Utc};

use crate::BiscuitSec;

pub(crate) async fn get_valid_token(
    root: Data<BiscuitSec>,
    name: web::Path<String>,
) -> impl Responder {
    let mut builder = Biscuit::builder(&root.keypair);
    builder
        .add_authority_fact(User(name.to_string()).as_biscuit_fact())
        .unwrap();

    builder
        .add_authority_check(
            format!(
                r#"check if time($time), $time < {}"#,
                (Utc::now() + Duration::seconds(600)).to_rfc3339()
            )
            .as_str(),
        )
        .unwrap();

    let biscuit = builder.build().unwrap();
    biscuit.to_base64().unwrap()
}

pub(crate) async fn hello(msg: ReqData<User>) -> impl Responder {
    format!("hello {:?}", *msg)
}

#[derive(Clone, Debug)]
pub(crate) struct User(String);

impl User {
    fn as_biscuit_fact(&self) -> Fact {
        Fact::new("user".to_string(), vec![Term::Str(self.0.to_string())])
    }

    fn from_authorizer(authorizer: &mut Authorizer) -> Result<Self, ()> {
        let res: Vec<(String,)> = authorizer
            .query("data($name) <- user($name)")
            .map_err(|_| ())?;
        Ok(User(res.get(0).ok_or(())?.0.clone()))
    }
}

pub(crate) async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
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
    authorizer.allow().map_err(|_| ())?;
    authorizer.authorize().map_err(|_| ())?;

    User::from_authorizer(&mut authorizer)
}
