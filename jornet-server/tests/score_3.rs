use jornet_server::domains::{
    admin::TokenReply,
    leaderboard::{Leaderboard, LeaderboardInput},
    player::{Player, PlayerInput},
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
async fn save_score_wrong_timestamp() {
    let app = helper::spawn_app().await;
    let client = reqwest::Client::new();

    let player = client
        .post(&format!("{}/api/v1/players", app.address))
        .json(&PlayerInput { name: None })
        .send()
        .await
        .expect("Failed to execute request.")
        .json::<Player>()
        .await
        .unwrap();

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
        .post(&format!("{}/api/v1/leaderboards", app.address))
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

    let mut score = ScoreInput::new(543.21, player, None, leaderboard.key);
    score.timestamp = 5120;
    let response = client
        .post(&format!("{}/api/v1/scores/{}", app.address, leaderboard.id))
        .json(&score)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_server_error());
}

#[tokio::test]
async fn save_score_repeat_score() {
    let app = helper::spawn_app().await;
    let client = reqwest::Client::new();

    let player = client
        .post(&format!("{}/api/v1/players", app.address))
        .json(&PlayerInput { name: None })
        .send()
        .await
        .expect("Failed to execute request.")
        .json::<Player>()
        .await
        .unwrap();

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
        .post(&format!("{}/api/v1/leaderboards", app.address))
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

    let score = ScoreInput::new(543.21, player, None, leaderboard.key);
    let response1 = client
        .post(&format!("{}/api/v1/scores/{}", app.address, leaderboard.id))
        .json(&score)
        .send()
        .await
        .expect("Failed to execute request.");
    let response2 = client
        .post(&format!("{}/api/v1/scores/{}", app.address, leaderboard.id))
        .json(&score)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response1.status().is_success());
    assert!(response2.status().is_server_error());
}
