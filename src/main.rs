mod errors;
mod models;
mod tests;

use crate::models::pokemon::{PokemonService, PokemonType};
use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, ResponseError};
#[get("/api/v1/pokemon/{name}")]
async fn get_pokemon(name: web::Path<String>) -> HttpResponse {
    let pk = PokemonService::new(name.to_lowercase().to_string(), PokemonType::BASIC);
    let handle = tokio::join!(pk.catch_pokemon());
    match handle {
        (Ok(t),) => HttpResponse::Ok().json(t),
        (Err(e),) => e.error_response(),
    }
}

#[get("/api/v1/pokemon/{name}/translated")]
async fn get_translated_pokemon(name: web::Path<String>) -> HttpResponse {
    let pk = PokemonService::new(name.to_lowercase().to_string(), PokemonType::TRANSLATED);
    let handle = tokio::join!(pk.catch_pokemon());
    match handle {
        (Ok(t),) => HttpResponse::Ok().json(t),
        (Err(e),) => e.error_response(),
    }
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .service(get_pokemon)
            .service(get_translated_pokemon)
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
