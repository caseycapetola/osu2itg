// osu!std file parser

use std::{collections::HashMap, fs::File, io::{Read, Write}, path::Path};

pub struct OsuParser {
    file: String,
    header_map: HashMap<String, String>,
}

impl OsuParser {
    pub fn new(file: String) -> Self {
        OsuParser {
            file,
            header_map: HashMap::new(),
        }
    }

    pub fn init_map(&mut self) {
        self.header_map.insert("Title".to_string(), "TITLE".to_string());
        self.header_map.insert("Artist".to_string(), "ARTIST".to_string());
        self.header_map.insert("Creator".to_string(), "CREDIT".to_string());
        self.header_map.insert("AudioFilename".to_string(), "MUSIC".to_string());
    }

    pub fn get_file(&self) -> String {
        self.file.clone()
    } 

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


    pub fn create_chart(&mut self, _data: &Vec<String>, folder_path: &str, song_title: &str) {
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
}