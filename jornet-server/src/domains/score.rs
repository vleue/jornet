use actix_cors::Cors;
use actix_web::{dev::HttpServiceFactory, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use time::{format_description::well_known::Rfc3339, UtcOffset};
use uuid::Uuid;

#[derive(Serialize)]
struct Score {
    score: f32,
    meta: Option<String>,
    timestamp: String,
    player: Uuid,
}

#[derive(Deserialize, Serialize)]
pub struct ScoreInput {
    pub score: f32,
    pub player: Uuid,
    pub meta: Option<String>,
}

async fn save_score(
    connection: web::Data<PgPool>,
    score: web::Json<ScoreInput>,
    leaderboard: web::Path<Uuid>,
) -> impl Responder {
    if Score::save(&score, &connection, &leaderboard).await {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

async fn get_scores(connection: web::Data<PgPool>, leaderboard: web::Path<Uuid>) -> impl Responder {
    HttpResponse::Ok().json(Score::get_all(&connection, &leaderboard).await)
}

pub(crate) fn score() -> impl HttpServiceFactory {
    let cors = Cors::default()
        .allow_any_header()
        .allow_any_origin()
        .allow_any_method()
        .send_wildcard()
        .max_age(3600);
    web::scope("api/scores")
        .wrap(cors)
        .route("{leaderboard_id}", web::post().to(save_score))
        .route("{leaderboard_id}", web::get().to(get_scores))
}

impl Score {
    pub async fn get_all(connection: &PgPool, leaderboard: &Uuid) -> Vec<Score> {
        sqlx::query!(
            "SELECT score, meta, timestamp, player FROM scores WHERE leaderboard = $1",
            leaderboard
        )
        .fetch_all(connection)
        .await
        .unwrap()
        .iter()
        .map(|r| Score {
            score: r.score,
            meta: r.meta.clone(),
            player: r.player.clone(),
            timestamp: r
                .timestamp
                .assume_offset(UtcOffset::UTC)
                .format(&Rfc3339)
                .unwrap(),
        })
        .collect()
    }

    pub async fn save(score: &ScoreInput, connection: &PgPool, leaderboard: &Uuid) -> bool {
        if sqlx::query!("SELECT id FROM leaderboards WHERE id = $1", leaderboard)
            .fetch_one(connection)
            .await
            .is_err()
        {
            return false;
        }
        sqlx::query!(
                r#"
            INSERT INTO scores (id, leaderboard, score, player, timestamp, meta) VALUES ($1, $2, $3, $4, NOW(), $5)
            "#,
                Uuid::new_v4(),
                leaderboard,
                score.score,
                score.player,
                score.meta,
            )
            .execute(connection)
            .await
        .is_ok()
    }
}
