use actix_web::{dev::HttpServiceFactory, web, HttpResponse, Responder};
use actix_web_httpauth::middleware::HttpAuthentication;
use biscuit_auth::KeyPair;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::auth_admin::validator;

use super::admin::AdminAccount;

#[derive(Deserialize)]
struct LeaderboardInput {
    name: String,
}

#[derive(Serialize)]
struct Leaderboard {
    id: Uuid,
    name: String,
}

async fn new_leaderboard(
    account: web::ReqData<AdminAccount>,
    connection: web::Data<PgPool>,
    leaderboard: web::Json<LeaderboardInput>,
) -> impl Responder {
    let leaderboard = Leaderboard {
        name: leaderboard.name.clone(),
        id: Uuid::new_v4(),
    };
    if leaderboard.create(&connection, account.id).await {
        HttpResponse::Ok().json(leaderboard)
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

async fn get_leaderboards(
    account: web::ReqData<AdminAccount>,
    connection: web::Data<PgPool>,
) -> impl Responder {
    HttpResponse::Ok().json(Leaderboard::get_all(&connection, account.id).await)
}

pub(crate) fn leaderboard(kp: web::Data<KeyPair>) -> impl HttpServiceFactory {
    web::scope("api/leaderboards")
        .app_data(kp)
        .wrap(HttpAuthentication::bearer(validator))
        .route("", web::post().to(new_leaderboard))
        .route("", web::get().to(get_leaderboards))
}

impl Leaderboard {
    pub async fn get_all(connection: &PgPool, owner: Uuid) -> Vec<Leaderboard> {
        sqlx::query!("SELECT id, name FROM leaderboards WHERE owner = $1", owner)
            .fetch_all(connection)
            .await
            .unwrap()
            .iter()
            .map(|r| Leaderboard {
                id: r.id,
                name: r.name.clone(),
            })
            .collect()
    }

    pub async fn create(&self, connection: &PgPool, owner: Uuid) -> bool {
        sqlx::query!(
            r#"
            INSERT INTO leaderboards (id, name, owner) VALUES ($1, $2, $3)
            "#,
            self.id,
            self.name,
            owner,
        )
        .execute(connection)
        .await
        .is_ok()
    }
}
