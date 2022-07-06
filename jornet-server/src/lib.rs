use std::{collections::HashMap, net::TcpListener};

use actix_web::{
    dev::Server,
    middleware::Logger,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder, Result,
};
use configuration::{get_configuration, Settings};
use serde::Deserialize;
use sqlx::PgPool;

pub mod configuration;
pub mod domains;

async fn index(config: web::Data<Settings>) -> impl Responder {
    HttpResponse::Ok().content_type("Text/Html").body(format!(
        r#"
<html>

<head>
    <title>Jornet Admin Panel</title>
</head>

<body><a href="https://github.com/login/oauth/authorize?client_id={}">Authenticate with GitHub</a>
</body>

</html>
    "#,
        config.github_admin_app.client_id
    ))
}

#[derive(Debug, Deserialize)]
pub struct OauthCode {
    code: String,
}

#[derive(Deserialize)]
pub struct GithubOauthResponse {
    access_token: String,
}

#[derive(Debug, Deserialize)]
pub struct GithubUser {
    login: String,
    id: u32,
}

async fn oauth_callback(
    code: web::Query<OauthCode>,
    config: web::Data<Settings>,
) -> impl Responder {
    let mut params = HashMap::new();
    params.insert("client_id", &config.github_admin_app.client_id);
    params.insert("client_secret", &config.github_admin_app.client_secret);
    params.insert("code", &code.code);

    let client = reqwest::Client::new();

    let github_bearer = client
        .post("https://github.com/login/oauth/access_token")
        .form(&params)
        .header("Accept", "application/json")
        .send()
        .await
        .unwrap()
        .json::<GithubOauthResponse>()
        .await
        .unwrap()
        .access_token;
    let user = client
        .get("https://api.github.com/user")
        .bearer_auth(github_bearer)
        .header("user-agent", "jornet")
        .send()
        .await
        .unwrap()
        .json::<GithubUser>()
        .await
        .unwrap();

    // TODO: redirect to another page, save a user in DB, add a biscuit
    format!("hello {}", user.login)
}

pub fn run(listener: TcpListener, connection_pool: PgPool) -> Result<Server, std::io::Error> {
    let config = Data::new(get_configuration());
    let root = Data::new(config.get_keypair());
    let connection = Data::new(connection_pool);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(connection.clone())
            .app_data(root.clone())
            .app_data(config.clone())
            .wrap(Logger::default())
            .route("/", web::get().to(index))
            .route("/oauth/callback", web::get().to(oauth_callback))
            .route(
                "/health_check",
                web::get().to(domains::healthcheck::health_check),
            )
            .service(domains::admins::admins(root.clone()))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
