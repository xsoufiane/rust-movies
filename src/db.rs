use std::collections::{HashMap, LinkedList};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use crate::{Genre, Movie};

struct DB {
    genre_db: HashMap<Genre, LinkedList<u32>>,
    movies_db: HashMap<u32, Movie>
}

impl DB {

    pub fn empty() -> Self {
        Self {
            genre_db: HashMap::new(),
            movies_db:  HashMap::new(),
        }
    }

    pub fn add(&mut self, movies: Vec<Movie>) -> &mut Self {
        for movie in movies {
            self.genre_db.insert(movie.id, movie);
            self.movies_db.insert(movie.id, movie);
        }

        self
    }

    pub fn init_from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;

        let reader = BufReader::new(file);

        let movies: Vec<Movie> = serde_json::from_reader(reader)?;

        ()
    }

    pub fn get_movies(&self, genre: &Genre) -> Option<&LinkedList<u32>> {
        self.db.get(genre)
    }

    pub fn get_movie(&self, id: &u32) -> Option<&Movie> {
        self.movies_db.get(id)
    }
}
