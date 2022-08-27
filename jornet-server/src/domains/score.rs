use std::time::{SystemTime, UNIX_EPOCH};

use actix_cors::Cors;
use actix_web::{dev::HttpServiceFactory, web, HttpResponse, Responder};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use sqlx::PgPool;
use time::{format_description::well_known::Rfc3339, UtcOffset};
use uuid::Uuid;

use super::{leaderboard::Leaderboard, player::Player};

#[derive(Serialize)]
struct Score {
    score: f32,
    meta: Option<String>,
    timestamp: String,
    player: String,
}

#[derive(Deserialize, Serialize)]
pub struct ScoreInput {
    pub score: f32,
    pub player: Uuid,
    pub meta: Option<String>,
    pub timestamp: u64,
    pub k: String,
}

impl ScoreInput {
    pub fn verify_mac(&self, key: Uuid, leaderboard_key: Uuid) -> bool {
        let mut mac = Hmac::<Sha256>::new_from_slice(key.as_bytes()).unwrap();
        mac.update(&self.timestamp.to_le_bytes());
        mac.update(leaderboard_key.as_bytes());
        mac.update(self.player.as_bytes());
        mac.update(&self.score.to_le_bytes());
        if let Some(meta) = self.meta.as_ref() {
            mac.update(meta.as_bytes());
        }
        mac.verify_slice(hex::decode(&self.k).unwrap().as_slice())
            .is_ok()
    }

    pub fn new(score: f32, player: Player, meta: Option<String>, leaderboard_key: Uuid) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let mut mac = Hmac::<Sha256>::new_from_slice(player.key.as_bytes()).unwrap();
        mac.update(&timestamp.to_le_bytes());
        mac.update(leaderboard_key.as_bytes());
        mac.update(player.id.as_bytes());
        mac.update(&score.to_le_bytes());
        if let Some(meta) = meta.as_ref() {
            mac.update(meta.as_bytes());
        }

        let hmac = hex::encode(&mac.finalize().into_bytes()[..]);
        Self {
            score,
            player: player.id,
            meta,
            timestamp,
            k: hmac,
        }
    }
}

async fn save_score(
    connection: web::Data<PgPool>,
    score: web::Json<ScoreInput>,
    leaderboard: web::Path<Uuid>,
) -> impl Responder {
    if let Some(player) = Player::get(score.player, &connection).await {
        if let Some(leaderboard_key) = Leaderboard::get_key(&connection, *leaderboard).await {
            if score.verify_mac(player.key, leaderboard_key)
                && Score::save(&score, &connection, &leaderboard).await
            {
                HttpResponse::Ok().json(())
            } else {
                HttpResponse::InternalServerError().finish()
            }
        } else {
            HttpResponse::InternalServerError().finish()
        }
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

async fn get_scores(connection: web::Data<PgPool>, leaderboard: web::Path<Uuid>) -> impl Responder {
    HttpResponse::Ok().json(Score::get_all(&connection, &leaderboard).await)
}

async fn delete_scores(
    connection: web::Data<PgPool>,
    leaderboard: web::Path<Uuid>,
) -> impl Responder {
    HttpResponse::Ok().json(Score::delete_all(&connection, &leaderboard).await)
}

pub(crate) fn score() -> impl HttpServiceFactory {
    let cors = Cors::default()
        .allow_any_header()
        .allow_any_origin()
        .allow_any_method()
        .send_wildcard()
        .max_age(3600);
    web::scope("api/v1/scores")
        .wrap(cors)
        .route("{leaderboard_id}", web::post().to(save_score))
        .route("{leaderboard_id}", web::get().to(get_scores))
        .route("{leaderboard_id}", web::delete().to(delete_scores))
}

impl Score {
    pub async fn get_all(connection: &PgPool, leaderboard: &Uuid) -> Vec<Score> {
        sqlx::query!(
            "SELECT score, meta, timestamp, players.name FROM scores, players WHERE leaderboard = $1 and scores.player = players.id",
            leaderboard
        )
        .fetch_all(connection)
        .await
        .unwrap()
        .iter()
        .map(|r| Score {
            score: r.score,
            meta: r.meta.clone(),
            player: r.name.clone(),
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

        if sqlx::query!(
            "SELECT id FROM scores WHERE leaderboard = $1 AND player = $2 AND score = $3 AND timestamp = TO_TIMESTAMP($4)",
            leaderboard,
            score.player,
            score.score,
            score.timestamp as f64
        )
            .fetch_one(connection)
            .await
            .is_ok()
        {
            return false;
        }

        sqlx::query!(
                r#"
            INSERT INTO scores (id, leaderboard, score, player, meta, timestamp) VALUES ($1, $2, $3, $4, $5, TO_TIMESTAMP($6))
            "#,
                Uuid::new_v4(),
                leaderboard,
                score.score,
                score.player,
                score.meta,
                score.timestamp as f64
            )
            .execute(connection)
            .await
        .is_ok()
    }

    pub async fn delete_all(connection: &PgPool, leaderboard: &Uuid) -> bool {
        sqlx::query!("DELETE FROM scores WHERE leaderboard = $1", leaderboard)
            .execute(connection)
            .await
            .is_ok()
    }
}
