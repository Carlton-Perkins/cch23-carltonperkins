mod d1;
mod n1;

use actix_web::{
    get,
    web::{scope, ServiceConfig},
};
use shuttle_actix_web::ShuttleActixWeb;

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
        cfg.service(n1::error);
        cfg.service(scope("/1").service(d1::d1));
    };

    Ok(config.into())
}
