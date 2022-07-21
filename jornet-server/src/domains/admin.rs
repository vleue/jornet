use actix_web::{dev::HttpServiceFactory, web, HttpResponse, Responder};
use actix_web_httpauth::middleware::HttpAuthentication;
use biscuit_auth::{
    builder::{Fact, Term},
    Authorizer, Biscuit, KeyPair,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use time::{format_description::well_known::Rfc3339, Duration, OffsetDateTime};
use uuid::Uuid;

use crate::auth_admin::validator;

use super::oauth::TOKEN_TTL;

#[derive(Serialize, Deserialize)]
pub struct TokenReply {
    pub token: String,
}

#[derive(Clone, Serialize)]
pub struct AdminAccount {
    pub id: Uuid,
}

pub trait BiscuitFact: Sized {
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

#[derive(Debug, Deserialize, Serialize)]
pub struct GithubUser {
    login: String,
    id: u32,
}

pub(crate) fn admin(kp: web::Data<KeyPair>) -> impl HttpServiceFactory {
    web::scope("api/admin")
        .app_data(kp)
        .wrap(HttpAuthentication::bearer(validator))
        .route("whoami", web::get().to(whoami))
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
    pub async fn exist(&self, connection: &PgPool) -> bool {
        sqlx::query!("SELECT id FROM admins WHERE id = $1", self.id)
            .fetch_one(connection)
            .await
            .is_ok()
    }
    pub async fn has_github(&self, connection: &PgPool) -> Option<GithubUser> {
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
    pub async fn create(&self, connection: &PgPool) -> bool {
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
    pub fn create_biscuit(&self, root: &KeyPair) -> Biscuit {
        let mut builder = Biscuit::builder(root);
        builder
            .add_authority_fact(AdminAccount { id: self.id }.as_biscuit_fact())
            .unwrap();

        builder
            .add_authority_check(
                format!(
                    r#"check if time($time), $time < {}"#,
                    (OffsetDateTime::now_utc() + Duration::seconds(TOKEN_TTL))
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
    pub async fn exist(&self, connection: &PgPool) -> bool {
        sqlx::query!(
            "SELECT id FROM admins_github WHERE id = $1 AND login = $2",
            self.id as i32,
            self.login
        )
        .fetch_one(connection)
        .await
        .is_ok()
    }
    pub async fn create(&self, account: &AdminAccount, connection: &PgPool) -> bool {
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
    pub async fn has_admin(&self, connection: &PgPool) -> Option<AdminAccount> {
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
