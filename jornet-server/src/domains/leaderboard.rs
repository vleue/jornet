use actix_web::{dev::HttpServiceFactory, web, HttpResponse, Responder};
use actix_web_httpauth::middleware::HttpAuthentication;
use biscuit_auth::KeyPair;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::auth_admin::validator;

use super::admin::AdminAccount;

#[derive(Deserialize, Serialize)]
pub struct LeaderboardInput {
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct Leaderboard {
    pub id: Uuid,
    pub key: Uuid,
    pub name: String,
}

#[derive(Serialize, Debug)]
struct LeaderboardWithScoreCount {
    id: Uuid,
    name: String,
    scores: i64,
}

async fn new_leaderboard(
    account: web::ReqData<AdminAccount>,
    connection: web::Data<PgPool>,
    leaderboard: web::Json<LeaderboardInput>,
) -> impl Responder {
    let leaderboard = Leaderboard {
        name: leaderboard.name.clone(),
        id: Uuid::new_v4(),
        key: Uuid::new_v4(),
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

async fn delete_all_scores(
    connection: web::Data<PgPool>,
    leaderboard: web::Path<Uuid>,
) -> impl Responder {
    HttpResponse::Ok().json(Leaderboard::delete_all_scores(&connection, &leaderboard).await)
}

pub(crate) fn leaderboard(kp: web::Data<KeyPair>) -> impl HttpServiceFactory {
    web::scope("api/v1/leaderboards")
        .app_data(kp)
        .wrap(HttpAuthentication::bearer(validator))
        .route("", web::post().to(new_leaderboard))
        .route("", web::get().to(get_leaderboards))
        .route(
            "{leaderboard_id}/scores",
            web::delete().to(delete_all_scores),
        )
}

impl Leaderboard {
    async fn get_all(connection: &PgPool, owner: Uuid) -> Vec<LeaderboardWithScoreCount> {
        sqlx::query!(
            "SELECT leaderboards.id, name, count(scores.leaderboard) FROM leaderboards LEFT JOIN scores ON leaderboards.id = scores.leaderboard WHERE owner = $1 GROUP BY leaderboards.id;",
            owner
        )
        .fetch_all(connection)
        .await
        .unwrap()
        .iter()
        .map(|r| LeaderboardWithScoreCount {
            id: r.id,
            name: r.name.clone(),
            scores: r.count.unwrap(),
        })
        .collect()
    }

    pub async fn get_key(connection: &PgPool, id: Uuid) -> Option<Uuid> {
        sqlx::query!("SELECT key FROM leaderboards WHERE id = $1;", id)
            .fetch_one(connection)
            .await
            .map(|r| r.key)
            .ok()
    }

    pub async fn create(&self, connection: &PgPool, owner: Uuid) -> bool {
        sqlx::query!(
            r#"
            INSERT INTO leaderboards (id, name, owner, key) VALUES ($1, $2, $3, $4)
            "#,
            self.id,
            self.name,
            owner,
            self.key,
        )
        .execute(connection)
        .await
        .is_ok()
    }

    pub async fn delete_all_scores(connection: &PgPool, leaderboard: &Uuid) -> bool {
        sqlx::query!("DELETE FROM scores WHERE leaderboard = $1", leaderboard)
            .execute(connection)
            .await
            .is_ok()
    }
}
