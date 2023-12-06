use actix_web::{
    post,
    web::{scope, Json, ServiceConfig},
};
use serde::{Deserialize, Serialize};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/6").service(count_elfs));
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
struct ElfCounts {
    elfs: usize,
    elf_shelves: usize,
    elf_not_shelves: usize,
}

#[post("/")]
async fn count_elfs(data: String) -> Json<ElfCounts> {
    let elfs = data.matches("elf").count();
    let elf_shelves = data.matches("elf on a shelf").count();
    let elf_not_shelves = data
        .match_indices("shelf")
        .filter(|(idx, _)| {
            let test = "elf on a ";
            let slice = &data[(idx - test.len()).max(0)..*idx];

            slice != test
        })
        .count();

    Json(ElfCounts {
        elfs,
        elf_shelves,
        elf_not_shelves,
    })
}

#[cfg(test)]
mod test {
    use actix_web::{test, App};

    use crate::d6::ElfCounts;

    #[actix_web::test]
    async fn elf_count_test() {
        let data = "The mischievous elf peeked out from behind the toy workshop,
        and another elf joined in the festive dance.
        Look, there is also an elf on that shelf!";
        let expected = 4;

        let app = test::init_service(App::new().service(super::count_elfs)).await;

        let req = test::TestRequest::post()
            .uri("/")
            .set_payload(data)
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
        assert_eq!(
            test::read_body_json::<super::ElfCounts, _>(res).await.elfs,
            expected
        );
    }

    #[test]
    async fn elf_shelf_count_test() {
        let data = "there is an elf on a shelf on an elf.
      there is also another shelf in Belfast.";
        let expected = ElfCounts {
            elfs: 5,
            elf_shelves: 1,
            elf_not_shelves: 1,
        };

        let app = test::init_service(App::new().service(super::count_elfs)).await;

        let req = test::TestRequest::post()
            .uri("/")
            .set_payload(data)
            .to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success());
        assert_eq!(
            test::read_body_json::<super::ElfCounts, _>(res).await,
            expected
        );
    }
}
