use actix_cors::Cors;
use actix_web::{dev::HttpServiceFactory, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
    pub key: Uuid,
}

#[derive(Deserialize, Serialize)]
pub struct PlayerInput {
    pub name: String,
}

async fn create_player(
    connection: web::Data<PgPool>,
    player: web::Json<PlayerInput>,
) -> impl Responder {
    let player = Player {
        name: player.name.clone(),
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
    web::scope("api/players")
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
}
