use actix_web::{dev::ServiceRequest, web, Error, HttpMessage, HttpResponse, Responder, Scope};
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
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct TokenReply {
    pub token: String,
}

#[derive(Clone)]
struct AdminAccount {
    id: Uuid,
}

trait BiscuitFact: Sized {
    fn as_biscuit_fact(&self) -> Fact;
    fn from_authorizer(authorizer: &mut Authorizer) -> Result<Self, ()>;
}

impl BiscuitFact for AdminAccount {
    fn as_biscuit_fact(&self) -> Fact {
        Fact::new("id".to_string(), vec![Term::Str(self.id.to_string())])
    }

    fn from_authorizer(authorizer: &mut Authorizer) -> Result<Self, ()> {
        let res: Vec<(String,)> = authorizer.query("data($id) <- id($id)").map_err(|_| ())?;
        Ok(AdminAccount {
            id: Uuid::parse_str(res.get(0).ok_or(())?.0.as_str()).map_err(|_| ())?,
        })
    }
}

async fn new_account(root: web::Data<KeyPair>, connection: web::Data<PgPool>) -> impl Responder {
    let uuid = Uuid::new_v4();
    match sqlx::query!(
        r#"
        INSERT INTO admins (id) VALUES ($1)
        "#,
        uuid,
    )
    .execute(connection.get_ref())
    .await
    {
        Ok(_) => {
            let mut builder = Biscuit::builder(&root);
            builder
                .add_authority_fact(AdminAccount { id: uuid }.as_biscuit_fact())
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
            HttpResponse::Ok().json(TokenReply {
                token: biscuit.to_base64().unwrap(),
            })
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    let root = req.app_data::<web::Data<KeyPair>>().unwrap();
    let biscuit = Biscuit::from_base64(credentials.token(), |_| root.public())
        .map_err(|_| AuthenticationError::from(Config::default()))?;

    let user = authorize(&biscuit).map_err(|_| AuthenticationError::from(Config::default()))?;

    req.extensions_mut().insert(user);
    Ok(req)
}

fn authorize(token: &Biscuit) -> Result<AdminAccount, ()> {
    let mut authorizer = token.authorizer().map_err(|_| ())?;

    authorizer.set_time();
    authorizer.allow().map_err(|_| ())?;
    authorizer.authorize().map_err(|_| ())?;

    AdminAccount::from_authorizer(&mut authorizer)
}

pub(crate) fn admins(kp: web::Data<KeyPair>) -> Scope {
    web::scope("")
        .route("auth/test", web::post().to(new_account))
        .service(
            web::scope("admin")
                .app_data(kp)
                .wrap(HttpAuthentication::bearer(validator))
                .route("hello", web::get().to(hello)),
        )
}

async fn hello(account: web::ReqData<AdminAccount>) -> impl Responder {
    format!("hello {:?}", account.id)
}
