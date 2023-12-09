use actix_web::{
    get,
    web::{self, scope, ServiceConfig},
};
use uom::{
    fmt::DisplayStyle,
    si::{
        acceleration::meter_per_second_squared, f32::*, length::meter, mass::hectogram,
        momentum::kilogram_meter_per_second,
    },
};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/8").service(weight));
}

#[derive(serde::Deserialize)]
struct Pokemon {
    weight: i32,
}

async fn get_pokemon(id: i32) -> Pokemon {
    reqwest::get(&format!("https://pokeapi.co/api/v2/pokemon/{id}/"))
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

#[get("/weight/{id}")]
async fn weight(path: web::Path<i32>) -> String {
    let id = path.into_inner();
    let data = get_pokemon(id).await;

    (data.weight as f32 / 10.).to_string()
}

#[get("/drop/{id}")]
async fn drop(path: web::Path<i32>) -> String {
    let id = path.into_inner();
    let data = get_pokemon(id).await;

    let poke_weight = Mass::new::<hectogram>(data.weight as f32);
    let gravity = Acceleration::new::<meter_per_second_squared>(9.825);
    let height = Length::new::<meter>(10.0);

    let final_vel: Velocity = (2. * (height * gravity)).sqrt();
    let momentum: Momentum = final_vel * poke_weight;

    let formatter = Momentum::format_args(kilogram_meter_per_second, DisplayStyle::Abbreviation);
    format!("{}", formatter.with(momentum).to_string())
        .split_whitespace()
        .collect::<Vec<&str>>()[0]
        .to_string()
}

#[cfg(test)]
mod test {
    use actix_web::{test, App};

    #[actix_web::test]
    async fn weight_test() {
        let data = "25";
        let expected = "6";

        let app = test::init_service(App::new().service(super::weight)).await;

        let req = test::TestRequest::get()
            .uri(&format!("/weight/{data}"))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
        assert_eq!(test::read_body(res).await, expected);
    }

    #[actix_web::test]
    async fn drop_test() {
        let data = "25";
        let expected = 84.10707461325713;

        let app = test::init_service(App::new().service(super::drop)).await;

        let req = test::TestRequest::get()
            .uri(&format!("/drop/{data}"))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
        let momentum = String::from_utf8_lossy(&test::read_body(res).await)
            .parse::<f64>()
            .unwrap();
        assert!(momentum - expected < 0.001);
    }
}
