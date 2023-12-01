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
