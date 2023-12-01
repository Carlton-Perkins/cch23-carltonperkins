use actix_web::{get, web::Path};

#[get("/{tail:.*}")]
pub async fn d1(path: Path<String>) -> String {
    let tail = path.into_inner();
    let parts = tail
        .split('/')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap());
    let res = parts.fold(0, |acc, x| acc ^ x).pow(3);

    res.to_string()
}

#[cfg(test)]
mod test {
    use actix_web::{test, App};

    #[actix_web::test]
    async fn d1_test() {
        let tests = vec![("/4/8", 1728), ("/10", 1000), ("/4/5/8/10", 27)];

        let app = test::init_service(App::new().service(super::d1)).await;
        for (path, expected) in tests {
            let req = test::TestRequest::get().uri(path).to_request();
            let res = test::call_service(&app, req).await;

            assert!(res.status().is_success());
            assert_eq!(test::read_body(res).await, expected.to_string());
        }
    }
}
