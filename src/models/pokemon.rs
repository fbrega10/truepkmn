use crate::errors::errors::PokeError;
use actix_web::web::Json;
use reqwest::{self, blocking::Client, StatusCode};
use serde::{Deserialize, Serialize};

const ENGLISH_LANGUAGE: &str = "en";

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonDto {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "habitat")]
    pub habitat: String,
    #[serde(rename = "is_legendary")]
    pub is_legendary: bool,
}

impl PokemonDto {
    pub fn new(
        name: String,
        description: String,
        habitat: String,
        is_legendary: bool,
    ) -> PokemonDto {
        PokemonDto {
            name,
            description,
            habitat,
            is_legendary,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PokemonType {
    TRANSLATED,
    BASIC,
}

//reusable object, it is repeated over the json data structure.
#[derive(Serialize, Deserialize, Debug)]
pub struct NameAndUrl {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "url")]
    pub url: String,
}

//represents the container for the pkmn description in any supported language
#[derive(Serialize, Deserialize, Debug)]
pub struct Flavor {
    #[serde(rename = "flavor_text")]
    pub flavor_text: String,
    #[serde(rename = "language")]
    pub language: NameAndUrl,
    #[serde(rename = "version")]
    pub version: NameAndUrl,
}

//it is the pokeapi.co response for the species uri
#[derive(Serialize, Deserialize, Debug)]
pub struct SpeciesResponse {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "habitat")]
    pub habitat: NameAndUrl,
    #[serde(rename = "is_legendary")]
    pub is_legendary: bool,
    #[serde(rename = "flavor_text_entries")]
    pub flavor_text_entries: Vec<Flavor>,
}

//service to be implemented
pub trait PokemonRetriever {
    fn catch_pokemon(&self) -> PokemonDto;
}

pub struct PokemonService {
    pub description: String,
    pub pokemon_type: PokemonType,
}

impl PokemonService {
    pub fn new(description: String, pokemon_type: PokemonType) -> PokemonService {
        PokemonService {
            description,
            pokemon_type,
        }
    }

    pub fn catch_pokemon(&self) -> Result<Json<PokemonDto>, PokeError> {
        let url = format!(
            "https://pokeapi.co/api/v2/pokemon-species/{}/",
            self.description
        );
        let client = Client::new();
        match client.get(url).send() {
            Ok(res) => {
                match res.status() {
                    StatusCode::NOT_FOUND => {
                        return Err(PokeError::NotFound("pokemon not found".to_string()))
                    }
                    StatusCode::OK => (),
                    _ => {
                        return Err(PokeError::ServiceUnavailable(
                            "unexpected error calling server pokeapi.co".to_string(),
                        ))
                    }
                }
                let species = res
                    .json::<SpeciesResponse>()
                    .expect("error reading the json Pokemon Species response");
                let habitat: String = (&species.habitat.name)
                    .chars()
                    .filter(|c| *c != '\\' && *c != '"')
                    .collect();
                println!("habitat : {}", habitat);
                let is_legendary: bool = *&species.is_legendary;
                let name = &species.name;
                let description: String = species
                    .flavor_text_entries
                    .into_iter()
                    .filter(|c| c.language.name.as_str() == ENGLISH_LANGUAGE)
                    .map(|c| c.flavor_text)
                    .last()
                    .expect("error extracting habitat from json pokeapi response")
                    .replace("\n", "");
                let pokemon: PokemonDto =
                    PokemonDto::new(name.clone(), description, habitat, is_legendary);
                println!("{:?}", pokemon);
                match self.pokemon_type {
                    PokemonType::BASIC => Ok(actix_web::web::Json(pokemon)),
                    PokemonType::TRANSLATED => Ok(actix_web::web::Json(pokemon)),
                }
            }
            Err(err) => {
                println!("error occurred calling pokeapi.co :  {}", err);
                Err(PokeError::ServiceUnavailable(
                    "pokeapi.co is currently unavailable ".to_string(),
                ))
            }
        }
    }
}

pub fn translate_pokemon(dto: PokemonDto) -> PokemonDto{

}