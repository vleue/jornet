use actix_web::{
    dev::ServiceRequest,
    web::{self, Data, ReqData},
    Error, HttpMessage, HttpResponse, Responder,
};
use actix_web_httpauth::extractors::{
    bearer::{BearerAuth, Config},
    AuthenticationError,
};
use biscuit_auth::{
    builder::{Fact, Term},
    Authorizer, Biscuit, KeyPair,
};
use chrono::{Duration, Utc};
use sqlx::PgPool;
use uuid::Uuid;

pub(crate) async fn get_valid_token(
    root: Data<KeyPair>,
    name: web::Path<String>,
    connection: web::Data<PgPool>,
) -> impl Responder {
    match sqlx::query!(
        r#"
        INSERT INTO admins (id, name) VALUES ($1, $2)
        "#,
        Uuid::new_v4(),
        name.as_str(),
    )
    .execute(connection.get_ref())
    .await
    {
        Ok(_) => {
            let mut builder = Biscuit::builder(&root);
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
            HttpResponse::Ok().body(biscuit.to_base64().unwrap())
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub(crate) async fn hello(user: ReqData<User>) -> impl Responder {
    format!("hello {:?}", *user)
}

pub(crate) async fn goodbye(user: ReqData<User>, connection: web::Data<PgPool>) -> impl Responder {
    match sqlx::query!(
        r#"
        DELETE FROM admins WHERE name = $1
        "#,
        user.0,
    )
    .execute(connection.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().body("goodbye"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
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
    let root = req.app_data::<Data<KeyPair>>().unwrap();
    let biscuit = Biscuit::from_base64(credentials.token(), |_| root.public())
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
