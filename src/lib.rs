mod genres_db;
mod movies_db;
mod types;

use crate::types::MovieId;
use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::HashSet;
pub use types::{Genre, Movie};

static DATA: &str = include_str!("db.json");

lazy_static! {
    static ref MOVIES_DATA: types::Data =
        serde_json::from_str(DATA).expect("failed to load json :(");
    static ref GENRES_DB: genres_db::DB = genres_db::DB::from_movies(MOVIES_DATA.movies.clone());
    static ref MOVIES_DB: movies_db::DB = movies_db::DB::from_movies(MOVIES_DATA.movies.clone());
}

/// assume non empty genres.
pub fn get_filtered_movies(genres: Vec<Genre>) -> Vec<types::Movie> {
    if genres.is_empty() {
        vec![MOVIES_DB.random().expect("no movies loaded...").clone()]
    } else {
        let mut sorted_movies: Vec<types::Movie> = Vec::new();
        let mut unique: HashSet<MovieId> = HashSet::new();

        for i in (1..=genres.len()).rev() {
            let combinations = genres.iter().combinations(i);

            for combination in combinations {
                MOVIES_DB
                    .filter_movies_by_genres(
                        GENRES_DB.get_movies_by_genres(&combination),
                        combination,
                    )
                    .iter()
                    .for_each(|movie| {
                        if !unique.contains(&movie.id) {
                            sorted_movies.push(movie.clone());
                            unique.insert(movie.id);
                        }
                    })
            }
        }

        sorted_movies
    }
}
