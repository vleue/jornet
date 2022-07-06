use jornet_server::domains::admins::TokenReply;

mod helper;

#[tokio::test]
async fn not_authenticated() {
    let app = helper::spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/admin/hello", app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(401, response.status());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn get_test_token() {
    let app = helper::spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .post(&format!("{}/auth/test", app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());

    sqlx::query!("SELECT id FROM admins",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");
}

#[tokio::test]
async fn use_test_token() {
    let app = helper::spawn_app().await;
    let client = reqwest::Client::new();

    let token = client
        .post(&format!("{}/auth/test", app.address))
        .send()
        .await
        .expect("Failed to execute request.")
        .json::<TokenReply>()
        .await
        .expect("got body");

    let response = client
        .get(&format!("{}/admin/whoami", app.address))
        .bearer_auth(token.token)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
}
