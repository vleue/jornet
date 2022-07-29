use actix_cors::Cors;
use actix_web::{dev::HttpServiceFactory, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::random_name::random_name;

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
    pub key: Uuid,
}

#[derive(Deserialize, Serialize)]
pub struct PlayerInput {
    pub name: Option<String>,
}

async fn create_player(
    connection: web::Data<PgPool>,
    player: web::Json<PlayerInput>,
) -> impl Responder {
    let player = Player {
        name: player.name.clone().unwrap_or_else(random_name),
        id: Uuid::new_v4(),
        key: Uuid::new_v4(),
    };

    if player.save(&connection).await {
        HttpResponse::Ok().json(player)
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

pub(crate) fn player() -> impl HttpServiceFactory {
    let cors = Cors::default()
        .allow_any_header()
        .allow_any_origin()
        .allow_any_method()
        .send_wildcard()
        .max_age(3600);
    web::scope("api/v1/players")
        .wrap(cors)
        .route("", web::post().to(create_player))
}

impl Player {
    pub async fn save(&self, connection: &PgPool) -> bool {
        sqlx::query!(
            r#"
            INSERT INTO players (id, name, key) VALUES ($1, $2, $3)
            "#,
            self.id,
            self.name,
            self.key,
        )
        .execute(connection)
        .await
        .is_ok()
    }

    pub async fn get(id: Uuid, connection: &PgPool) -> Option<Player> {
        sqlx::query!(
            r#"
            SELECT id, name, key FROM players WHERE id = $1
            "#,
            id,
        )
        .fetch_one(connection)
        .await
        .map(|r| Player {
            id: r.id,
            name: r.name.clone(),
            key: r.key,
        })
        .ok()
    }
}
