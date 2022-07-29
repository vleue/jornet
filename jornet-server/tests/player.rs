use jornet_server::domains::player::{Player, PlayerInput};
use serde::Serialize;
use uuid::Uuid;

mod helper;

#[derive(Serialize)]
struct UuidInput {
    uuid: Uuid,
}

#[tokio::test]
async fn create_player() {
    let app = helper::spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .post(&format!("{}/api/players", app.address))
        .json(&PlayerInput {
            name: Some("hello".to_string()),
        })
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    let _player: Player = response.json().await.unwrap();
}

#[tokio::test]
async fn create_player_with_random_name() {
    let app = helper::spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .post(&format!("{}/api/players", app.address))
        .json(&PlayerInput { name: None })
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    let player: Player = response.json().await.unwrap();
    assert!(!player.name.is_empty());
}
