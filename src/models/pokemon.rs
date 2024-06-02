use crate::errors::errors::PokeError;
use actix_web::web::Json;
use log::{debug, error, info, log_enabled, Level};
use reqwest::{self, blocking::Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

const ENGLISH_LANGUAGE: &str = "en";
const YODA_URL: &str = "https://api.funtranslations.com/translate/yoda.json";
const SHAKESPEARE_URL: &str = "https://api.funtranslations.com/translate/shakespeare.json";
const POKE_NOT_FOUND: &str = "Error: pokemon not found!";

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
    pub fn set_description(&mut self, description: String) {
        self.description = description
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Translation {
    #[serde(rename = "contents")]
    pub contents: Contents,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Contents {
    #[serde(rename = "translated")]
    pub translated: String,
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
                        error!("{}", POKE_NOT_FOUND);
                        return Err(PokeError::NotFound("pokemon not found".to_string()));
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
                    .expect("error parsing the json Pokemon Species response");
                let habitat: String = (&species.habitat.name)
                    .chars()
                    .filter(|c| *c != '\\' && *c != '"')
                    .collect();
                info!("current pokemon habitat : {}", habitat);
                let is_legendary: bool = *&species.is_legendary;
                let name = &species.name;
                let description: String = species
                    .flavor_text_entries
                    .into_iter()
                    .filter(|c| c.language.name.as_str() == ENGLISH_LANGUAGE)
                    .map(|c| c.flavor_text)
                    .last()
                    .expect("error extracting habitat from json pokeapi response")
                    .replace("\n", " ");
                let mut pokemon: PokemonDto =
                    PokemonDto::new(name.clone(), description, habitat, is_legendary);
                match self.pokemon_type {
                    PokemonType::BASIC => Ok(Json(pokemon)),
                    PokemonType::TRANSLATED => {
                        translate_pokemon(&mut pokemon);
                        Ok(Json(pokemon))
                    }
                }
            }
            Err(err) => {
                error!("error occurred calling pokeapi.co : {}", err);
                Err(PokeError::ServiceUnavailable(
                    "pokeapi.co is currently unavailable ".to_string(),
                ))
            }
        }
    }
}

pub fn translate_pokemon(dto: &mut PokemonDto) {
    //call to the funapitranslations server and get the formatted description
    //using a reference to modify (no copying in memory) the pokemonDto value
    info!("pokemon dto to be translated : {:?}", dto);
    let url = if dto.habitat.to_string() == "cave".to_string() || dto.is_legendary {
        info!("YODA url selected!");
        YODA_URL
    } else {
        info!("SHAKESPEARE url selected!");
        SHAKESPEARE_URL
    };
    let client = Client::new();
    let content = String::from(&dto.description);
    let params = ("text", content.as_str());
    info!(
        "query params used in the translation server call {:?}",
        &params
    );
    match client.get(url).query(&[params]).send() {
        Ok(t) => {
            println!("{:?}", t);
            match t.status() {
                StatusCode::OK => {
                    let response = t
                        .json::<Translation>()
                        .expect("error parsing the json response from shakespeare")
                        .contents
                        .translated;
                    dto.set_description(response);
                }
                status => error!("invalid status code, server responded with : {}", status),
            }
        }
        Err(e) => error!("error calling the translation API? {}", e),
    }
}
