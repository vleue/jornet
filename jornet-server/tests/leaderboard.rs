use jornet_server::domains::{admin::TokenReply, leaderboard::LeaderboardInput};
use serde::Serialize;
use uuid::Uuid;

mod helper;

#[derive(Serialize)]
struct UuidInput {
    uuid: Uuid,
}

#[tokio::test]
async fn create_leaderboard() {
    let app = helper::spawn_app().await;
    let client = reqwest::Client::new();

    let token = client
        .post(&format!("{}/oauth/by_uuid", app.address))
        .json(&UuidInput {
            uuid: Uuid::new_v4(),
        })
        .send()
        .await
        .expect("Failed to execute request.")
        .json::<TokenReply>()
        .await
        .expect("got body");

    let response = client
        .post(&format!("{}/api/leaderboards", app.address))
        .bearer_auth(token.token)
        .json(&LeaderboardInput {
            name: "my leaderboard".to_string(),
        })
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
}
