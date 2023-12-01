use actix_web::{get, HttpResponse};

#[get("/-1/error")]
pub async fn error() -> HttpResponse {
    HttpResponse::InternalServerError().finish()
}
