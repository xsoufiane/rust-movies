use crate::types::MovieId;
use crate::{Genre, Movie};
use rand::Rng;
use std::collections::{HashMap, HashSet};

pub struct DB {
    movies_db: HashMap<MovieId, Movie>,
}

impl DB {
    pub fn empty() -> Self {
        Self {
            movies_db: HashMap::new(),
        }
    }

    pub fn from_movies(movies: Vec<Movie>) -> Self {
        let mut movies_db = DB::empty();
        for movie in movies {
            movies_db.add(movie)
        }

        movies_db
    }

    pub fn add(&mut self, movie: Movie) {
        self.movies_db.insert(movie.id, movie);
    }

    pub fn get_movie(&self, id: &MovieId) -> Option<&Movie> {
        self.movies_db.get(id)
    }

    pub fn random(&self) -> Option<&Movie> {
        if self.movies_db.is_empty() {
            None
        } else {
            let mut rng = rand::thread_rng();
            let random_key = self
                .movies_db
                .keys()
                .nth(rng.gen_range(0..self.movies_db.len()))
                .expect("Oops this shouldn't happen...");

            self.movies_db.get(random_key)
        }
    }

    pub fn filter_movies_by_genres(
        &self,
        movies: HashSet<MovieId>,
        genres: Vec<&Genre>,
    ) -> Vec<Movie> {
        let mut genres: Vec<Genre> = genres.into_iter().cloned().collect();
        genres.sort();

        let mut filtered_movies: Vec<Movie> = movies
            .iter()
            .filter_map(|id| match self.get_movie(id) {
                None => {
                    tracing::info!("no movie for id: {}", id);
                    None
                }
                Some(movie) => {
                    let mut movie_genres = movie.genres.clone();
                    movie_genres.sort();

                    if movie_genres == genres {
                        Some(movie.clone())
                    } else {
                        None
                    }
                }
            })
            .collect();

        filtered_movies.sort_by(|a, b| a.id.cmp(&b.id));

        filtered_movies
    }
}
