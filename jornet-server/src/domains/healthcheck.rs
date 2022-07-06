use actix_web::{HttpResponse, Responder};

pub(crate) async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}
