use std::collections::HashMap;

use actix_web::{
    get,
    web::{scope, ServiceConfig},
    HttpRequest,
};
use base64::Engine;
use serde_json::json;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/7").service(decode));
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Cookie {
    recipe: HashMap<String, usize>,
    pantry: HashMap<String, usize>,
}

#[get("/decode")]
async fn decode(request: HttpRequest) -> String {
    let header = request.headers().get("Cookie").unwrap().to_str().unwrap();
    let header_str = header.to_string();
    let cookie = header_str.split_once('=').unwrap().1;
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(cookie)
        .unwrap();

    String::from_utf8(decoded).unwrap()
}

#[get("/bake")]
async fn bake(request: HttpRequest) -> String {
    let header = request.headers().get("Cookie").unwrap().to_str().unwrap();
    let header_str = header.to_string();
    let cookie = header_str.split_once('=').unwrap().1;
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(cookie)
        .unwrap();
    let decoded_str = String::from_utf8(decoded).unwrap();
    let cookie: Cookie = serde_json::from_str(&decoded_str).unwrap();

    let mut cookies = 0;
    let recipe = cookie.recipe;
    let mut pantry = cookie.pantry.clone();

    'done: loop {
        let mut new_pantry = pantry.clone();
        for (ingredient, amount) in recipe.iter() {
            let pantry_amount = new_pantry.entry(ingredient.clone()).or_default();
            if *pantry_amount < *amount {
                break 'done;
            }

            *pantry_amount -= *amount;
        }
        cookies += 1;
        pantry = new_pantry;
    }

    json!({ "cookies": cookies, "pantry": pantry }).to_string()
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use actix_web::{test, App};
    use serde_json::Value;

    #[actix_web::test]
    async fn decode_test() {
        let data = "recipe=eyJmbG91ciI6MTAwLCJjaG9jb2xhdGUgY2hpcHMiOjIwfQ==";
        let expected = r#"{"flour":100,"chocolate chips":20}"#;

        let app = test::init_service(App::new().service(super::decode)).await;

        let req = test::TestRequest::get()
            .uri("/decode")
            .append_header(("Cookie", data))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
        assert_eq!(test::read_body(res).await, expected);
    }

    #[test]
    async fn bake_test() {
        let data = "recipe=eyJyZWNpcGUiOnsiZmxvdXIiOjk1LCJzdWdhciI6NTAsImJ1dHRlciI6MzAsImJha2luZyBwb3dkZXIiOjEwLCJjaG9jb2xhdGUgY2hpcHMiOjUwfSwicGFudHJ5Ijp7ImZsb3VyIjozODUsInN1Z2FyIjo1MDcsImJ1dHRlciI6MjEyMiwiYmFraW5nIHBvd2RlciI6ODY1LCJjaG9jb2xhdGUgY2hpcHMiOjQ1N319";
        let expected_cookies = 4;
        let expected_pantry = HashMap::from([
            ("flour", 5),
            ("sugar", 307),
            ("butter", 2002),
            ("baking powder", 825),
            ("chocolate chips", 257),
        ]);

        let app = test::init_service(App::new().service(super::bake)).await;

        let req = test::TestRequest::get()
            .uri("/bake")
            .append_header(("Cookie", data))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
        let body = test::read_body_json::<Value, _>(res).await;
        assert_eq!(body["cookies"], expected_cookies);
        let pantry = body["pantry"].as_object().unwrap();
        for (ingredient, amount) in expected_pantry.iter() {
            assert_eq!(pantry.get(*ingredient).unwrap(), *amount);
        }
    }

    #[test]
    async fn bad_bake_test() {
        let data =
            "recipe=eyJyZWNpcGUiOnsic2xpbWUiOjl9LCJwYW50cnkiOnsiY29iYmxlc3RvbmUiOjY0LCJzdGljayI6IDR9fQ==";
        let expected_cookies = 0;
        let expected_pantry = HashMap::from([("cobblestone", 64), ("stick", 4)]);

        let app = test::init_service(App::new().service(super::bake)).await;

        let req = test::TestRequest::get()
            .uri("/bake")
            .append_header(("Cookie", data))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
        let body = test::read_body_json::<Value, _>(res).await;
        assert_eq!(body["cookies"], expected_cookies);
        let pantry = body["pantry"].as_object().unwrap();
        for (ingredient, amount) in expected_pantry.iter() {
            assert_eq!(pantry.get(*ingredient).unwrap(), *amount);
        }
    }
}
