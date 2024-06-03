use crate::errors::errors::PokeError;
use actix_web::web::Json;
use log::{error, info};
use reqwest::{self, blocking::Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

const ENGLISH_LANGUAGE: &str = "en";
const YODA_URL: &str = "https://api.funtranslations.com/translate/yoda.json";
const SHAKESPEARE_URL: &str = "https://api.funtranslations.com/translate/shakespeare.json";
const POKE_NOT_FOUND: &str = "Error: pokemon not found!";

/*
The pokemon Data transfer object: it is the output of the service
 */
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
    /*
    Utility to set a field, sugar syntax, coming from a OOP programmer (not needed in this case)
     */
    pub fn set_description(&mut self, description: String) {
        self.description = description
    }
}

/*
the flow of the program execution depends on this enum :
having it to a TRANSLATED value means we want to change
the description to the YODA or SHAKESPEARE language, depending
on the pokemon habitat/legendary status.
 */

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
        /*
        catch_pokemon is the main method of the truepkmn library:
        having a service with a description field it utilizes this string
        to get all the information needed to serve a pokemon dto object as a response
         */
        let url = format!(
            "https://pokeapi.co/api/v2/pokemon-species/{}/",
            self.description
        );
        let client = Client::new();
        match client.get(url).send() {
            Ok(res) => {
                match res.status() {
                    //pokemon not found: returning the custom Error struct
                    StatusCode::NOT_FOUND => {
                        error!("{}", POKE_NOT_FOUND);
                        return Err(PokeError::NotFound);
                    }
                    StatusCode::OK => (),
                    e => {
                        error!("error occurred, status code : {}", e);
                        return Err(PokeError::ServiceUnavailable);
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
            //An error occurred trying to connect to the remote pokeapi.co server: return an Error object
            Err(err) => {
                error!("error occurred calling pokeapi.co : {}", err);
                Err(PokeError::ServiceUnavailable)
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
                //log any error: check if the status code is != from 200 (OK), do nothing
                status => error!("invalid status code, server responded with : {}", status),
            }
        }
        //log any error coming from the funapitranslations server: do nothing, keep the original description
        Err(e) => error!("error calling the translation API? {}", e),
    }
}
