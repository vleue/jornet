use actix_web::{web, HttpResponse, Responder, Scope};
use serde::Serialize;

use crate::configuration::Settings;

#[derive(Debug, Serialize)]
pub struct OauthConfig {
    github_app_id: String,
}

async fn get_oauth_config(config: web::Data<Settings>) -> impl Responder {
    HttpResponse::Ok().json(OauthConfig {
        github_app_id: config.github_admin_app.client_id.clone(),
    })
}

pub(crate) fn config(config: web::Data<Settings>) -> Scope {
    web::scope("api/config")
        .app_data(config)
        .route("oauth", web::get().to(get_oauth_config))
}
