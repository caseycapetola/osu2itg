#[derive(Debug, Clone)]
pub struct Metadata {
    pub title: String, // Romanised song title
    pub title_unicode: String, // Song title
    pub artist: String, // Romanised song artist
    pub artist_unicode: String, // Song artist
    pub creator: String, // Beatmap creator
    pub version: String, // Difficulty name
    pub source: String, // Original media the song was produced for
    pub tags: Vec<String>, // Search terms
    pub beatmap_id: i32, // Difficulty ID
    pub beatmap_set_id: i32, // Beatmap ID
}