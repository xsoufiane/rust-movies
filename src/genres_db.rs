use crate::types::MovieId;
use crate::{Genre, Movie};
use std::collections::{HashMap, HashSet};

pub struct DB {
    genres_db: HashMap<Genre, HashSet<MovieId>>,
}

impl DB {
    pub fn empty() -> Self {
        Self {
            genres_db: HashMap::new(),
        }
    }

    pub fn from_movies(movies: Vec<Movie>) -> Self {
        let mut genres_db = DB::empty();
        for movie in movies {
            genres_db.add(movie);
        }

        genres_db
    }

    pub fn add(&mut self, movie: Movie) -> &mut Self {
        for genre in movie.genres {
            self.genres_db
                .entry(genre)
                .and_modify(|movies| {
                    movies.insert(movie.id);
                })
                .or_insert(HashSet::from([movie.id]));
        }

        self
    }

    pub fn get_movies_by_genre(&self, genre: &Genre) -> Option<&HashSet<MovieId>> {
        self.genres_db.get(genre)
    }

    /// set of movies that are in one of the genres
    pub fn get_movies_by_genres(&self, genres: &Vec<&Genre>) -> HashSet<&MovieId> {
        let mut movies_set: HashSet<&MovieId> = HashSet::new();

        for genre in genres {
            match self.get_movies_by_genre(genre) {
                None => tracing::info!("no movies for the {} genre", genre),
                Some(movies) => movies.iter().for_each(|movie| {
                    movies_set.insert(movie);
                }),
            }
        }

        movies_set
    }
}
