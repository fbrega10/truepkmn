mod errors;
mod models;

use crate::errors::errors::PokeError;
use crate::models::pokemon::{PokemonDto, PokemonService, PokemonType};
use actix_web::web::Json;
use actix_web::{get, web, App, HttpResponse, HttpServer, ResponseError};
use tokio::task::JoinHandle;

#[get("/api/v1/pokemon/{name}")]
async fn get_pokemon(name: web::Path<String>) -> HttpResponse {
    let pk = PokemonService::new(name.to_string(), PokemonType::BASIC);
    let handle: JoinHandle<Result<Json<PokemonDto>, PokeError>> =
        tokio::task::spawn_blocking(move || pk.catch_pokemon());
    match handle.await.unwrap() {
        Ok(t) => HttpResponse::Ok().json(t),
        Err(e) => e.error_response(),
    }
}

#[get("/api/v1/pokemon/{name}/translated")]
async fn get_translated_pokemon(name: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_pokemon))
        .service(get_translated_pokemon)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
