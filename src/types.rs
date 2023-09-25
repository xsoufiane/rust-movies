pub struct Movie {
    pub id: u32,
    pub title: String,
    pub year: String,
    pub runtime: String,
    pub genres: Vec<Genre>,
    pub director: String,
    pub actors: String,
    pub plot: String,
    pub poster_url: String,
}

pub type Genre = String;