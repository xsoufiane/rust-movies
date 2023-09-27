use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Movie {
    pub id: MovieId,
    pub title: String,
    pub year: String,
    pub runtime: String,
    pub genres: Vec<Genre>,
    pub director: String,
    pub actors: String,
    pub plot: String,
    #[serde(rename = "posterUrl")]
    pub poster_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub genres: HashSet<Genre>,
    pub movies: Vec<Movie>,
}

pub type Genre = String;

pub type MovieId = u32;
