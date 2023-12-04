use actix_web::{
    get,
    web::{scope, Json, ServiceConfig},
};
use serde::{Deserialize, Serialize};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/4").service(strength).service(contest));
}

#[derive(serde::Deserialize, PartialEq, PartialOrd)]
struct Reindeer {
    name: String,
    strength: i32,
    #[serde(default)]
    speed: f64,
    #[serde(default)]
    height: i32,
    #[serde(default)]
    antler_width: i32,
    #[serde(default)]
    snow_magic_power: i32,
    #[serde(default)]
    favorite_food: String,
    #[serde(default, rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies: i32,
}

#[get("/strength")]
async fn strength(reindeers: Json<Vec<Reindeer>>) -> String {
    reindeers
        .iter()
        .map(|r| r.strength)
        .sum::<i32>()
        .to_string()
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
struct ContestResults {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

impl ContestResults {
    fn new(reindeers: Vec<Reindeer>) -> Self {
        let fastest = reindeers
            .iter()
            .max_by(|l, r| l.speed.total_cmp(&r.speed))
            .unwrap();
        let tallest = reindeers.iter().max_by_key(|r| r.height).unwrap();
        let magician = reindeers.iter().max_by_key(|r| r.snow_magic_power).unwrap();
        let consumer = reindeers.iter().max_by_key(|r| r.candies).unwrap();

        Self {
            fastest: format!(
                "Speeding past the finish line with a strength of {} is {}",
                fastest.strength, fastest.name
            ),
            tallest: format!(
                "{} is standing tall with his {} cm wide antlers",
                tallest.name, tallest.antler_width
            ),
            magician: format!(
                "{} could blast you away with a snow magic power of {}",
                magician.name, magician.snow_magic_power
            ),
            consumer: format!("{} ate lots of candies, but also some grass", consumer.name),
        }
    }
}

#[get("/contest")]
async fn contest(reindeers: Json<Vec<Reindeer>>) -> Json<ContestResults> {
    Json(ContestResults::new(reindeers.into_inner()))
}

#[cfg(test)]
mod test {
    use actix_web::{test, App};
    use serde_json::json;

    #[actix_web::test]
    async fn strength_test() {
        let data = json! {[
          { "name": "Dasher", "strength": 5 },
          { "name": "Dancer", "strength": 6 },
          { "name": "Prancer", "strength": 4 },
          { "name": "Vixen", "strength": 7 }
        ]};
        let expected = 22;

        let app = test::init_service(App::new().service(super::strength)).await;

        let req = test::TestRequest::get()
            .uri("/strength")
            .set_json(data)
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
        assert_eq!(test::read_body(res).await, expected.to_string());
    }

    #[actix_web::test]
    async fn contest_test() {
        let data = json! {[
        {
          "name": "Dasher",
          "strength": 5,
          "speed": 50.4,
          "height": 80,
          "antler_width": 36,
          "snow_magic_power": 9001,
          "favorite_food": "hay",
          "cAnD13s_3ATeN-yesT3rdAy": 2
        },
        {
          "name": "Dancer",
          "strength": 6,
          "speed": 48.2,
          "height": 65,
          "antler_width": 37,
          "snow_magic_power": 4004,
          "favorite_food": "grass",
          "cAnD13s_3ATeN-yesT3rdAy": 5
        }]};
        let expected = super::ContestResults {
            fastest: "Speeding past the finish line with a strength of 5 is Dasher".to_string(),
            tallest: "Dasher is standing tall with his 36 cm wide antlers".to_string(),
            magician: "Dasher could blast you away with a snow magic power of 9001".to_string(),
            consumer: "Dancer ate lots of candies, but also some grass".to_string(),
        };

        let app = test::init_service(App::new().service(super::contest)).await;

        let req = test::TestRequest::get()
            .uri("/contest")
            .set_json(data)
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
        assert_eq!(
            test::read_body_json::<super::ContestResults, _>(res).await,
            expected
        );
    }
}
