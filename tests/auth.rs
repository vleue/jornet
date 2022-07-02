use std::net::TcpListener;

#[tokio::test]
async fn not_authenticated() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/api/hello", address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(401, response.status());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn get_test_token() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/get_valid_token/hola", address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
}

#[tokio::test]
async fn use_test_token() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let token = client
        .get(&format!("{}/get_valid_token/hola", address))
        .send()
        .await
        .expect("Failed to execute request.")
        .text()
        .await
        .expect("got body");

    let response = client
        .get(&format!("{}/api/hello", address))
        .bearer_auth(token)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = jornet::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
