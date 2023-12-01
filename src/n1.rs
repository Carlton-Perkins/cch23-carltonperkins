use actix_web::{get, HttpResponse};

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[get("/-1/error")]
pub async fn error() -> HttpResponse {
    HttpResponse::InternalServerError().finish()
}

#[cfg(test)]
mod test {
    use actix_web::{test, App};

    #[actix_web::test]
    async fn n1_1_test() {
        let app = test::init_service(App::new().service(super::hello_world)).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
    }

    #[actix_web::test]
    async fn n1_2_test() {
        let app = test::init_service(App::new().service(super::error)).await;
        let req = test::TestRequest::get().uri("/-1/error").to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_server_error());
    }
}
