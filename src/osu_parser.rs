// osu!std file parser

use std::{fs::File, io::{Read, Write}, path::Path};

pub struct OsuParser {
    file: String,
}

impl OsuParser {
    pub fn new(file: String) -> Self {
        OsuParser {
            file,
        }
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

    pub fn parse_file(&mut self) -> Vec<String> {
        let data = self.read_file();
        let collect = data.lines().map(|line| line.to_string()).collect::<Vec<String>>();
        // for line in collect.iter() {
        //     println!("{} ###", line);
        // }
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

    pub fn _get_general(&mut self, data: &Vec<String>) -> Vec<String> {
        let mut song_details = vec![String::new()];
        let mut iter = data.iter();

        while let Some(line) = iter.next() {
            if line.contains("[General]") {
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

    pub fn _get_hit_objects(&mut self, data: &Vec<String>) -> Vec<String> {
        let mut hit_objects = vec![String::new()];
        let mut iter = data.iter();

        while let Some(line) = iter.next() {
            if line.contains("[HitObjects]") {
                while let Some(hit_object_line) = iter.next() {
                    hit_objects.push(hit_object_line.clone());
                }
            }
        }
        hit_objects
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