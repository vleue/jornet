use std::collections::HashMap;

use actix_web::{web, HttpResponse, Responder, Scope};
use biscuit_auth::{
    builder::{Fact, Term},
    Authorizer, Biscuit, KeyPair,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::configuration::Settings;

use super::admins::{AdminAccount, GithubUser};

pub const TOKEN_TTL: i64 = 600;

#[derive(Serialize, Deserialize)]
pub struct TokenReply {
    pub token: String,
}

trait BiscuitFact: Sized {
    fn as_biscuit_fact(&self) -> Fact;
    fn from_authorizer(authorizer: &mut Authorizer) -> Option<Self>;
}

impl BiscuitFact for AdminAccount {
    fn as_biscuit_fact(&self) -> Fact {
        Fact::new("user".to_string(), vec![Term::Str(self.id.to_string())])
    }

    fn from_authorizer(authorizer: &mut Authorizer) -> Option<Self> {
        let res: Vec<(String,)> = authorizer.query("data($id) <- user($id)").ok()?;
        Some(AdminAccount {
            id: Uuid::parse_str(res.get(0)?.0.as_str()).ok()?,
        })
    }
}

#[derive(Deserialize)]
struct UuidInput {
    uuid: Uuid,
}

async fn by_uuid(
    root: web::Data<KeyPair>,
    connection: web::Data<PgPool>,
    uuid: web::Json<UuidInput>,
) -> impl Responder {
    let account = AdminAccount { id: uuid.uuid };
    match (
        account.exist(&connection).await,
        account.has_github(&connection).await,
    ) {
        (_, Some(_)) => return HttpResponse::InternalServerError().finish(),
        (false, _) => {
            account.create(&connection).await;
        }
        (true, _) => (),
    }

    let biscuit = account.create_biscuit(root.as_ref());
    HttpResponse::Ok().json(TokenReply {
        token: biscuit.to_base64().unwrap(),
    })
}
pub fn authorize(token: &Biscuit) -> Option<AdminAccount> {
    let mut authorizer = token.authorizer().ok()?;

    authorizer.set_time();
    authorizer.allow().map_err(|_| ()).ok()?;
    authorizer.authorize().map_err(|_| ()).ok()?;

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

    let admin = if user.exist(&connection).await {
        user.has_admin(&connection).await.unwrap()
    } else {
        let account = AdminAccount { id: Uuid::new_v4() };
        account.create(&connection).await;
        user.create(&account, &connection).await;
        account
    };

    let biscuit = admin.create_biscuit(&root);
    HttpResponse::Ok().json(TokenReply {
        token: biscuit.to_base64().unwrap(),
    })
}

pub(crate) fn oauth() -> Scope {
    web::scope("oauth")
        .route("by_uuid", web::post().to(by_uuid))
        .route("callback", web::get().to(oauth_callback))
}

#[derive(Serialize)]
struct Identity<'a> {
    admin: &'a AdminAccount,
    github: Option<GithubUser>,
}
