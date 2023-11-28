use actix_web::{get, web::ServiceConfig, HttpResponse};

#[get("/-1/error")]
pub async fn error() -> HttpResponse {
    HttpResponse::InternalServerError().finish()
}
