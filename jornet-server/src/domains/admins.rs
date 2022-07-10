use std::collections::HashMap;

use actix_web::{
    cookie::{
        time::{format_description::well_known::Rfc3339, Duration, OffsetDateTime},
        Cookie,
    },
    dev::ServiceRequest,
    web, Error, HttpMessage, HttpResponse, Responder, Scope,
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
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::configuration::Settings;

use super::admin_site::AUTH_COOKIE_KEY;

#[derive(Serialize, Deserialize)]
pub struct TokenReply {
    pub token: String,
}

#[derive(Clone, Serialize)]
pub struct AdminAccount {
    id: Uuid,
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

async fn by_uuid_get(
    root: web::Data<KeyPair>,
    connection: web::Data<PgPool>,
    uuid: web::Query<UuidInput>,
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
    HttpResponse::Found()
        .cookie(
            Cookie::build(AUTH_COOKIE_KEY, biscuit.to_base64().unwrap())
                .secure(true)
                .expires(OffsetDateTime::now_utc() + Duration::seconds(600))
                .path("/admin/")
                .finish(),
        )
        .insert_header(("Location", "/admin/"))
        .finish()
}

async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    let root = req.app_data::<web::Data<KeyPair>>().unwrap();
    let biscuit = Biscuit::from_base64(credentials.token(), |_| root.public())
        .map_err(|_| AuthenticationError::from(Config::default()))?;

    let user = authorize(&biscuit).ok_or_else(|| AuthenticationError::from(Config::default()))?;

    req.extensions_mut().insert(user);
    Ok(req)
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

#[derive(Debug, Deserialize, Serialize)]
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

    let admin = if user.exist(&connection).await {
        user.has_admin(&connection).await.unwrap()
    } else {
        let account = AdminAccount { id: Uuid::new_v4() };
        account.create(&connection).await;
        user.create(&account, &connection).await;
        account
    };

    let biscuit = admin.create_biscuit(&root);

    HttpResponse::Found()
        .cookie(
            Cookie::build(AUTH_COOKIE_KEY, biscuit.to_base64().unwrap())
                .secure(true)
                .expires(OffsetDateTime::now_utc() + Duration::seconds(600))
                .path("/admin/")
                .finish(),
        )
        .insert_header(("Location", "/admin/"))
        .finish()
}

pub(crate) fn admins(kp: web::Data<KeyPair>) -> Scope {
    web::scope("")
        .route("auth/by_uuid", web::post().to(by_uuid))
        .route("auth/by_uuid", web::get().to(by_uuid_get))
        .route("/oauth/callback", web::get().to(oauth_callback))
        .service(
            web::scope("api/admin")
                .app_data(kp)
                .wrap(HttpAuthentication::bearer(validator))
                .route("whoami", web::get().to(whoami)),
        )
}

#[derive(Serialize)]
struct Identity<'a> {
    admin: &'a AdminAccount,
    github: Option<GithubUser>,
}

async fn whoami(
    account: web::ReqData<AdminAccount>,
    connection: web::Data<PgPool>,
) -> impl Responder {
    HttpResponse::Ok().json(Identity {
        admin: &account,
        github: account.has_github(&connection).await,
    })
}

impl AdminAccount {
    async fn exist(&self, connection: &PgPool) -> bool {
        sqlx::query!("SELECT id FROM admins WHERE id = $1", self.id)
            .fetch_one(connection)
            .await
            .is_ok()
    }
    async fn has_github(&self, connection: &PgPool) -> Option<GithubUser> {
        match sqlx::query!(
            "SELECT id, login FROM admins_github WHERE admin_id = $1",
            self.id
        )
        .fetch_one(connection)
        .await
        {
            Ok(record) => Some(GithubUser {
                login: record.login,
                id: record.id as u32,
            }),
            _ => None,
        }
    }
    async fn create(&self, connection: &PgPool) -> bool {
        sqlx::query!(
            r#"
            INSERT INTO admins (id) VALUES ($1)
            "#,
            self.id,
        )
        .execute(connection)
        .await
        .is_ok()
    }
    fn create_biscuit(&self, root: &KeyPair) -> Biscuit {
        let mut builder = Biscuit::builder(root);
        builder
            .add_authority_fact(AdminAccount { id: self.id }.as_biscuit_fact())
            .unwrap();

        builder
            .add_authority_check(
                format!(
                    r#"check if time($time), $time < {}"#,
                    (OffsetDateTime::now_utc() + Duration::seconds(600))
                        .format(&Rfc3339)
                        .unwrap()
                )
                .as_str(),
            )
            .unwrap();

        builder.build().unwrap()
    }
}

impl GithubUser {
    async fn exist(&self, connection: &PgPool) -> bool {
        sqlx::query!(
            "SELECT id FROM admins_github WHERE id = $1 AND login = $2",
            self.id as i32,
            self.login
        )
        .fetch_one(connection)
        .await
        .is_ok()
    }
    async fn create(&self, account: &AdminAccount, connection: &PgPool) -> bool {
        sqlx::query!(
            r#"
            INSERT INTO admins_github (id, login, admin_id) VALUES ($1, $2, $3)
            "#,
            self.id as i64,
            self.login,
            account.id,
        )
        .fetch_one(connection)
        .await
        .is_ok()
    }
    async fn has_admin(&self, connection: &PgPool) -> Option<AdminAccount> {
        sqlx::query!(
            "SELECT admin_id FROM admins_github WHERE id = $1 AND login = $2",
            self.id as i32,
            self.login
        )
        .fetch_one(connection)
        .await
        .map(|record| AdminAccount {
            id: record.admin_id,
        })
        .ok()
    }
}
