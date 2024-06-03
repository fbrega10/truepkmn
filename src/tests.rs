#[cfg(test)]
mod tests {
    use crate::get_pokemon;
    use crate::get_translated_pokemon;
    use actix_web::{test, App};
    use mockito::{Matcher, Mock, Server};
    use tokio::task::JoinHandle;

    const SHAKESPEARE_URL: &str = "https://api.funtranslations.com/translate/shakespeare.json";
    const YODA_URL: &str = "https://api.funtranslations.com/translate/yoda.json";
    const POKEAPI: &str = "https://pokeapi.co/api/v2/pokemon-species/charizard/";
    const TRANSLATED_URI: &str = "/api/v1/pokemon/charizard/translated";
    const BASIC_URI: &str = "/api/v1/pokemon/charizard";
    const GET_METHOD: &str = "GET";
    const POST_METHOD: &str = "POST";
    const CHARIZARD_DESCRIPTION: &str = "Spits fire that is hot enough to melt boulders.Known to cause forest fires unintentionally.";
    const CHARIZARD_RAW_BODY: &str = "{\"base_happiness\": 50, \"capture_rate\": 45, \"color\": { \"name\": \"red\", \"url\": \"https://pokeapi.co/api/v2/pokemon-color/8/\" }, \"flavor_text_entries\": [ { \"flavor_text\": \"Spits fire that\nis hot enough to\nmelt boulders.Known to cause\nforest fires\nunintentionally.\", \"language\": { \"name\": \"en\", \"url\": \"https://pokeapi.co/api/v2/language/9/\" }, \"version\": { \"name\": \"red\", \"url\": \"https://pokeapi.co/api/v2/version/1/\" } } ], \"habitat\": { \"name\": \"mountain\", \"url\": \"https://pokeapi.co/api/v2/pokemon-habitat/4/\" }, \"has_gender_differences\": false, \"hatch_counter\": 20, \"id\": 6, \"is_legendary\": false, \"is_mythical\": false, \"name\": \"charizard\" }";
    const LEGENDARY_RAW_BODY: &str = "{\"base_happiness\": 50, \"capture_rate\": 45, \"color\": { \"name\": \"red\", \"url\": \"https://pokeapi.co/api/v2/pokemon-color/8/\" }, \"flavor_text_entries\": [ { \"flavor_text\": \"Spits fire that\nis hot enough to\nmelt boulders.Known to cause\nforest fires\nunintentionally.\", \"language\": { \"name\": \"en\", \"url\": \"https://pokeapi.co/api/v2/language/9/\" }, \"version\": { \"name\": \"red\", \"url\": \"https://pokeapi.co/api/v2/version/1/\" } } ], \"habitat\": { \"name\": \"cave\", \"url\": \"https://pokeapi.co/api/v2/pokemon-habitat/4/\" }, \"has_gender_differences\": false, \"hatch_counter\": 20, \"id\": 6, \"is_legendary\": true, \"is_mythical\": false, \"name\": \"charizard\" }";
    #[actix_web::test]
    async fn test_not_found() {
        let handle: JoinHandle<Mock> = tokio::task::spawn_blocking(move || {
            Server::new().mock(GET_METHOD, POKEAPI).with_status(404)
        });
        let app = test::init_service(App::new().service(get_pokemon)).await;
        let req = test::TestRequest::get()
            .uri("/api/v1/pokemon/charizardss")
            .to_request();
        let resp = test::call_service(&app, req).await;
        handle.await.ok();
        assert!(resp.status().is_client_error());
    }

    #[actix_web::test]
    async fn test_ok_basic() {
        let handle: JoinHandle<Mock> = tokio::task::spawn_blocking(move || {
            Server::new()
                .mock(GET_METHOD, POKEAPI)
                .with_body(CHARIZARD_RAW_BODY)
                .with_status(200)
        });
        let app = test::init_service(App::new().service(get_pokemon)).await;
        let req = test::TestRequest::get().uri(BASIC_URI).to_request();
        let resp = test::call_service(&app, req).await;
        handle.await.ok();
        assert!(resp.status().is_success());
        assert_eq!(200, resp.status().as_u16())
    }

    #[actix_web::test]
    async fn test_ok_translated() {
        let handle: JoinHandle<Mock> = tokio::task::spawn_blocking(move || {
            Server::new()
                .mock(GET_METHOD, POKEAPI)
                .with_body(CHARIZARD_RAW_BODY)
                .with_status(200);
            Server::new()
                .mock(POST_METHOD, YODA_URL)
                .match_query(Matcher::UrlEncoded(
                    "text".into(),
                    CHARIZARD_DESCRIPTION.into(),
                ))
                .with_status(200)
        });
        let app = test::init_service(App::new().service(get_translated_pokemon)).await;
        let req = test::TestRequest::get().uri(TRANSLATED_URI).to_request();
        let resp = test::call_service(&app, req).await;
        handle.await.ok();
        assert!(resp.status().is_success());
        assert_eq!(200, resp.status().as_u16())
    }

    #[actix_web::test]
    async fn test_ok_translated_legendary() {
        let handle: JoinHandle<Mock> = tokio::task::spawn_blocking(move || {
            Server::new()
                .mock(GET_METHOD, POKEAPI)
                .with_body(LEGENDARY_RAW_BODY)
                .with_status(200);
            Server::new()
                .mock(POST_METHOD, SHAKESPEARE_URL)
                .match_query(Matcher::UrlEncoded(
                    "text".into(),
                    CHARIZARD_DESCRIPTION.into(),
                ))
                .with_status(200)
        });
        let app = test::init_service(App::new().service(get_translated_pokemon)).await;
        let req = test::TestRequest::get().uri(TRANSLATED_URI).to_request();
        let resp = test::call_service(&app, req).await;
        handle.await.ok();
        assert!(resp.status().is_success());
        assert_eq!(200, resp.status().as_u16());
    }
    #[actix_web::test]
    async fn test_ko_translated_legendary() {
        let handle: JoinHandle<Mock> = tokio::task::spawn_blocking(move || {
            Server::new().mock(GET_METHOD, POKEAPI)
                .with_body("{\"base_happiness\": 50, \"capture_rate\": 45, \"color\": { \"name\": \"red\", \"url\": \"https://pokeapi.co/api/v2/pokemon-color/8/\" }, \"flavor_text_entries\": [ { \"flavor_text\": \"Spits fire that\nis hot enough to\nmelt boulders.Known to cause\nforest fires\nunintentionally.\", \"language\": { \"name\": \"en\", \"url\": \"https://pokeapi.co/api/v2/language/9/\" }, \"version\": { \"name\": \"red\", \"url\": \"https://pokeapi.co/api/v2/version/1/\" } } ], \"habitat\": { \"name\": \"cave\", \"url\": \"https://pokeapi.co/api/v2/pokemon-habitat/4/\" }, \"has_gender_differences\": false, \"hatch_counter\": 20, \"id\": 6, \"is_legendary\": true, \"is_mythical\": false, \"name\": \"charizard\" }")
                .with_status(200);
            Server::new()
                .mock(POST_METHOD, SHAKESPEARE_URL)
                .match_query(Matcher::UrlEncoded(
                    "text".into(),
                    CHARIZARD_DESCRIPTION.into(),
                ))
                .with_status(503)
        });
        let app = test::init_service(App::new().service(get_translated_pokemon)).await;
        let req = test::TestRequest::get().uri(TRANSLATED_URI).to_request();
        let resp = test::call_service(&app, req).await;
        handle.await.ok();
        assert!(resp.status().is_success());
        assert_eq!(200, resp.status().as_u16())
    }
}
