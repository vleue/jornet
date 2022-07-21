use jornet_server::domains::{
    admin::TokenReply,
    leaderboard::{Leaderboard, LeaderboardInput},
    score::ScoreInput,
};
use serde::Serialize;
use uuid::Uuid;

mod helper;

#[derive(Serialize)]
struct UuidInput {
    uuid: Uuid,
}

#[tokio::test]
async fn save_score() {
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

    let leaderboard = client
        .post(&format!("{}/api/leaderboards", app.address))
        .bearer_auth(token.token)
        .json(&LeaderboardInput {
            name: "my leaderboard".to_string(),
        })
        .send()
        .await
        .expect("Failed to execute request.")
        .json::<Leaderboard>()
        .await
        .expect("valid leaderboard");

    let response = client
        .post(&format!("{}/api/scores/{}", app.address, leaderboard.id))
        .json(&ScoreInput {
            score: 543.21,
            player: Uuid::new_v4(),
            meta: None,
        })
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
}

#[tokio::test]
async fn save_score_to_missing_dashboard() {
    let app = helper::spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .post(&format!("{}/api/scores/{}", app.address, Uuid::new_v4()))
        .json(&ScoreInput {
            score: 543.21,
            player: Uuid::new_v4(),
            meta: None,
        })
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_server_error());
}
