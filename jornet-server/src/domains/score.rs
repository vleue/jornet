use actix_web::{dev::HttpServiceFactory, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
struct Score {
    score: f32,
    player: Uuid,
    meta: Option<String>,
}

async fn save_score(
    connection: web::Data<PgPool>,
    score: web::Json<Score>,
    leaderboard: web::Path<Uuid>,
) -> impl Responder {
    if score.save(&connection, &leaderboard).await {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

async fn get_scores(connection: web::Data<PgPool>, leaderboard: web::Path<Uuid>) -> impl Responder {
    HttpResponse::Ok().json(Score::get_all(&connection, &leaderboard).await)
}

pub(crate) fn score() -> impl HttpServiceFactory {
    web::scope("api/scores")
        .route("{leaderboard_id}", web::post().to(save_score))
        .route("{leaderboard_id}", web::get().to(get_scores))
}

impl Score {
    pub async fn get_all(connection: &PgPool, leaderboard: &Uuid) -> Vec<Score> {
        sqlx::query!(
            "SELECT score, player, meta FROM scores WHERE leaderboard = $1",
            leaderboard
        )
        .fetch_all(connection)
        .await
        .unwrap()
        .iter()
        .map(|r| Score {
            score: r.score,
            player: r.player.clone(),
            meta: r.meta.clone(),
        })
        .collect()
    }

    pub async fn save(&self, connection: &PgPool, leaderboard: &Uuid) -> bool {
        if !sqlx::query!("SELECT id FROM leaderboards WHERE id = $1", leaderboard)
            .fetch_one(connection)
            .await
            .is_ok()
        {
            return false;
        }
        sqlx::query!(
                r#"
            INSERT INTO scores (id, leaderboard, score, player, timestamp, meta) VALUES ($1, $2, $3, $4, NOW(), $5)
            "#,
                Uuid::new_v4(),
                leaderboard,
                self.score,
                self.player,
                self.meta,
            )
            .execute(connection)
            .await
        .is_ok()
    }
}
