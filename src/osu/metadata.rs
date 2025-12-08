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

impl Metadata {
    fn default() -> Self {
        Self {
            title: String::new(),
            title_unicode: String::new(),
            artist: String::new(),
            artist_unicode: String::new(),
            creator: String::new(),
            version: String::new(),
            source: String::new(),
            tags: Vec::new(),
            beatmap_id: 0,
            beatmap_set_id: 0,
        }
    }

    pub fn new(data: String) -> Self {
        let mut metadata = Metadata::default();

        for line in data.lines() {
            // Check for header value
            if line.contains("[") && !line.contains("Metadata") {
                println!("Issue with parsing Metadata section, exiting parse.");
                break;
            } else if line.trim().is_empty() || line.contains("[") || line.starts_with("//") {
                continue;
            }
            // Parse key-value pairs
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() != 2 {
                continue;
            }
            let key = parts[0].trim();
            let value = parts[1].trim();
            match key {
                "Title" => metadata.title = value.to_string(),
                "TitleUnicode" => metadata.title_unicode = value.to_string(),
                "Artist" => metadata.artist = value.to_string(),
                "ArtistUnicode" => metadata.artist_unicode = value.to_string(),
                "Creator" => metadata.creator = value.to_string(),
                "Version" => metadata.version = value.to_string(),
                "Source" => metadata.source = value.to_string(),
                "Tags" => metadata.tags = value.split_whitespace().map(|s| s.to_string()).collect(),
                "BeatmapID" => metadata.beatmap_id = value.parse().unwrap_or(0),
                "BeatmapSetID" => metadata.beatmap_set_id = value.parse().unwrap_or(0),
                _ => {}
            }
        }
        return metadata;
    }
}