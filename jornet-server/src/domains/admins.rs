use std::collections::HashMap;

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

use crate::configuration::Settings;

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
        Fact::new("user".to_string(), vec![Term::Str(self.id.to_string())])
    }

    fn from_authorizer(authorizer: &mut Authorizer) -> Result<Self, ()> {
        let res: Vec<(String,)> = authorizer.query("data($id) <- user($id)").map_err(|_| ())?;
        Ok(AdminAccount {
            id: Uuid::parse_str(res.get(0).ok_or(())?.0.as_str()).map_err(|_| ())?,
        })
    }
}

fn create_biscuit(uuid: Uuid, root: &KeyPair) -> Biscuit {
    let mut builder = Biscuit::builder(root);
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

    builder.build().unwrap()
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
            let biscuit = create_biscuit(uuid, &root);

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

#[derive(Debug, Deserialize)]
pub struct OauthCode {
    code: String,
}

#[derive(Deserialize)]
pub struct GithubOauthResponse {
    access_token: String,
}

#[derive(Debug, Deserialize)]
pub struct GithubUser {
    login: String,
    id: u32,
}

async fn oauth_callback(
    code: web::Query<OauthCode>,
    config: web::Data<Settings>,
    connection: web::Data<PgPool>,
    root: web::Data<KeyPair>,
) -> impl Responder {
    let mut params = HashMap::new();
    params.insert("client_id", &config.github_admin_app.client_id);
    params.insert("client_secret", &config.github_admin_app.client_secret);
    params.insert("code", &code.code);

    let client = reqwest::Client::new();

    let github_bearer = client
        .post("https://github.com/login/oauth/access_token")
        .form(&params)
        .header("Accept", "application/json")
        .send()
        .await
        .unwrap()
        .json::<GithubOauthResponse>()
        .await
        .unwrap()
        .access_token;
    let user = client
        .get("https://api.github.com/user")
        .bearer_auth(github_bearer)
        .header("user-agent", "jornet")
        .send()
        .await
        .unwrap()
        .json::<GithubUser>()
        .await
        .unwrap();

    let uuid = match sqlx::query!("SELECT admin_id FROM admins_github",)
        .fetch_one(connection.get_ref())
        .await
    {
        Ok(record) => record.admin_id,
        _ => {
            let uuid = Uuid::new_v4();
            sqlx::query!(
                r#"
                INSERT INTO admins (id) VALUES ($1)
                "#,
                uuid,
            )
            .execute(connection.get_ref())
            .await
            .unwrap();
            sqlx::query!(
                r#"
                INSERT INTO admins_github (id, login, admin_id) VALUES ($1, $2, $3)
                "#,
                user.id as i64,
                user.login,
                uuid,
            )
            .execute(connection.get_ref())
            .await
            .unwrap();
            uuid
        }
    };
    // TODO: redirect to another page, save a user in DB, add a biscuit
    let biscuit = create_biscuit(uuid, &root);

    HttpResponse::Ok().json(TokenReply {
        token: biscuit.to_base64().unwrap(),
    })
}

pub(crate) fn admins(kp: web::Data<KeyPair>) -> Scope {
    web::scope("")
        .route("auth/test", web::post().to(new_account))
        .route("/oauth/callback", web::get().to(oauth_callback))
        .service(
            web::scope("admin")
                .app_data(kp)
                .wrap(HttpAuthentication::bearer(validator))
                .route("whoami", web::get().to(whoami)),
        )
}

async fn whoami(
    account: web::ReqData<AdminAccount>,
    connection: web::Data<PgPool>,
) -> impl Responder {
    match sqlx::query!(
        "SELECT login FROM admins_github where admin_id = $1",
        account.id
    )
    .fetch_one(connection.get_ref())
    .await
    {
        Ok(record) => format!(
            "hello {:?}. Connected with your GitHub account {}",
            account.id, record.login
        ),
        _ => format!("hello {:?}. You don't have an account linked", account.id),
    }
}
