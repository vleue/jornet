mod helper;

#[tokio::test]
async fn not_authenticated() {
    let app = helper::spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/api/hello", app.address))
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
        .get(&format!("{}/get_valid_token/hola", app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());

    let saved = sqlx::query!("SELECT id, name FROM admins",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");
    assert_eq!(saved.name, "hola");
}

#[tokio::test]
async fn use_test_token() {
    let app = helper::spawn_app().await;
    let client = reqwest::Client::new();

    let token = client
        .get(&format!("{}/get_valid_token/hola", app.address))
        .send()
        .await
        .expect("Failed to execute request.")
        .text()
        .await
        .expect("got body");

    let response = client
        .get(&format!("{}/api/hello", app.address))
        .bearer_auth(token)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
}
