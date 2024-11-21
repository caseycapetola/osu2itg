mod osu_parser;
mod file_tools;

use crate::osu_parser::OsuParser;

struct Song {
    title: String,
    subtitle: String,
    artist: String,
    credit: String, // simfile author
    music: String,
    banner: String,
    background: String,
    sample_start: String,
    sample_length: String,
}

impl Song {
    pub fn new(title: String, artist: String, credit: String, subtitle: String) -> Song {
        Song {
            title,
            subtitle,
            artist,
            credit,
            music: String::with_capacity(128),
            banner: String::with_capacity(128),
            background: String::with_capacity(128),
            sample_start: String::with_capacity(128),
            sample_length: String::with_capacity(128),
        }
    }

    pub fn get_song_details(&self) -> String {
        format!("Title: {}\nSubtitle: {}\nArtist: {}\nCredit: {}\nMusic: {}\nBanner: {}\nBackground: {}\nSample Start: {}\nSample Length: {}", 
            self.title, self.subtitle, self.artist, self.credit, self.music, self.banner, self.background, self.sample_start, self.sample_length)
    }

}

fn main() {
    let mut parser = OsuParser::new("assets/Wings_of_Justice/woj_reduced.osu".to_string());
    println!("Hello, world!");
    let file_data = parser.parse_file();
    for i in file_data.iter() {
        println!("{}", i);
        println!("###");
    }
    let song_details = parser.get_metadata(&file_data);
    for line in song_details.iter() {
        println!("{}", line);
    };
    println!("---------------------------");

    let song = Song::new("WINGS OF JUSTICE".to_string(), "GALNERYUS".to_string(), "Sotarks".to_string(), "FLYING TOWARDS JUSTICE".to_string());
    println!("{}", song.get_song_details());
    // parser.create_chart(&file_data, "/Projects/osu2itg/test", "testing");
    println!("\n\n\n---------------------------");
    parser.write_chart(&file_data, "E:\\Projects\\osu2itg\\test.sm");
    println!("{}", parser.calc_bpm(&file_data));
}
