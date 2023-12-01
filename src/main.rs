mod d1;
mod n1;

use actix_web::web::{scope, ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(n1::hello_world);
        cfg.service(n1::error);
        cfg.service(scope("/1").service(d1::d1));
    };

    Ok(config.into())
}
