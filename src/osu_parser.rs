// osu!std file parser
use std::{fs::File, io::{Read, Write}, path::{Path, PathBuf}}; //, collections::HashMap};
use crate::file_tools::{Deserialize, OsuAudioFilename, OsuTitle, SM5AudioFilename, SM5Title};

pub struct OsuParser {
    file: String,
    // _header_map: HashMap<String, String>,
}

impl OsuParser {
    pub fn new(file: String) -> Self {
        OsuParser {
            file,
            // header_map: HashMap::new(),
        }
    }

    // pub _fn init_map(&mut self) {
    //     self.header_map.insert("Title".to_string(), "TITLE".to_string());
    //     self.header_map.insert("Artist".to_string(), "ARTIST".to_string());
    //     self.header_map.insert("Creator".to_string(), "CREDIT".to_string());
    //     self.header_map.insert("AudioFilename".to_string(), "MUSIC".to_string());
    // }

    // pub fn _get_file(&self) -> String {
    //     self.file.clone()
    // } 

    // Reads file into string
    fn read_file(&mut self) -> String {
        let mut f = File::open(self.file.clone()).expect("Unable to open file");
        let mut data = String::new();
        f.read_to_string(&mut data).expect("Unable to read data");
        return data;
    }

    // Splits file by [Sections]
    pub fn parse_file(&mut self) -> Vec<String> {
        let data = self.read_file();
        let collect = data.split("\r\n\r\n").map(|s| s.to_string()).collect::<Vec<String>>();
        return collect;

    }

    // Write fields to chart file
    pub fn write_chart(&mut self, osu_data: &Vec<String>, file: &str) {
        const OSU_FIELDS: [&str; 2] = ["Title", "AudioFilename"];

        // let mut file = File::create(file).expect("Unable to create file");
        let binding = file.to_string();
        let path = Path::new(&binding);
        let display = path.display();

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };
        for i in osu_data.iter() {
            // SKIP OVER TIMING FOR NOW TO SIMPLIFY IMPLEMENTATION
            if i.contains("[TimingPoints]") || i.contains("[HitObjects]") || i.contains("[Events]") {
                continue;
            }
            
            // Remove header to only have key-value pairs
            let fields = i.split("\r\n").collect::<Vec<&str>>();
            for j in fields.iter() {
                // Split key value on ":"
                let parts: Vec<&str> = j.split(":").collect();
                if parts.len() == 2 {
                    let key = parts[0].trim();
                    let value = parts[1].trim();
                    if OSU_FIELDS.contains(&key) {
                        // Process key and value
                        if key == "AudioFilename" {
                            let osu_field = OsuAudioFilename { name: PathBuf::from(value) };
                            let sm5_audio_filename: SM5AudioFilename = From::from(osu_field);
                            file.write(sm5_audio_filename.deserialize().as_bytes()).expect("Unable to write data");
                        }
                        else if key == "Title" {
                            let osu_field = OsuTitle { name: PathBuf::from(value) };
                            let sm5_title: SM5Title = From::from(osu_field);
                            file.write(sm5_title.deserialize().as_bytes()).expect("Unable to write data");
                        }
                        
                    }
                }
            }

        }

        file.write_all(b"").expect("Unable to write data");
    }

    // RELEVANT FIELDS (for ITG): Title, Artist, Creator, Version
    pub fn get_metadata(&mut self, data: &Vec<String>) -> Vec<String> {
        let mut song_details = vec![String::new()];
        let mut iter = data.iter();

        while let Some(line) = iter.next() {
            if line.contains("[Metadata]") {
                while let Some(metadata_line) = iter.next() {
                    if metadata_line.contains("[") {
                        break;
                    }
                    song_details.push(metadata_line.clone());
                }
            }
        }
        song_details
    }


    pub fn _create_chart(&mut self, _data: &Vec<String>, folder_path: &str, song_title: &str) {
        let filepath = Path::new(folder_path);
        let prefix = filepath.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();
        let binding = "{}.ssc".replace("{}", song_title);
        let path = Path::new(&binding);
        let display = path.display();

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };
        file.write("#TITLE:{};\n".replace("{}", song_title).as_bytes()).expect("Unable to write data");
        


    }

    pub fn calc_bpm(&self, data: &Vec<String>) -> f32 {
        let mut iter = data.iter();
        let mut bpm = 0.0;
        while let Some(line) = iter.next() {
            if line.contains("[TimingPoints]") {
                let timing_info = line.split("\r\n").collect::<Vec<&str>>();
                for i in timing_info.iter() {
                    if i.contains("[") {
                        continue;
                    }
                    let timing_data = i.split(",").collect::<Vec<&str>>();
                    if timing_data[6] == "1" {
                        bpm = 60000.0 / timing_data[1].parse::<f32>().unwrap();
                    }
                }
            }
        }
        bpm
    }
}