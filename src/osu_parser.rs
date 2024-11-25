// osu!std file parser
use std::{fs::{File, Metadata}, io::{Read, Write}, path::{Path, PathBuf}}; //, collections::HashMap};
use crate::file_tools::{Deserialize, OsuAudioFilename, OsuPreviewTime, OsuTitle, SM5AudioFilename, SM5PreviewTime, SM5Title};

#[derive(Clone)]
#[derive(Debug)]
pub enum OsuHeader {
    General(Vec<String>),
    Editor(Vec<String>),
    Metadata(Vec<String>),
    Difficulty(Vec<String>),
    Events(Vec<String>),
    TimingPoints(Vec<String>),
    Colours(Vec<String>),
    HitObjects(Vec<String>),
}

pub struct OsuParser {
    file: String,
    pub general: OsuHeader,
    editor: OsuHeader,
    metadata: OsuHeader,
    difficulty: OsuHeader,
    events: OsuHeader,
    timing_points: OsuHeader,
    colours: OsuHeader,
    hit_objects: OsuHeader,
}

// Parse osu! file headers
fn parse_headers(file: String) -> [OsuHeader; 8] {
    let mut f = File::open(file.clone()).expect("Unable to open file");
    let mut data = String::new();
    f.read_to_string(&mut data).expect("Unable to read data");
    let collect = data.split("\r\n\r\n").map(|s| s.to_string()).collect::<Vec<String>>();
    let mut headers: [OsuHeader; 8] = [
        OsuHeader::General(vec![]),
        OsuHeader::Editor(vec![]),
        OsuHeader::Metadata(vec![]),
        OsuHeader::Difficulty(vec![]),
        OsuHeader::Events(vec![]),
        OsuHeader::TimingPoints(vec![]),
        OsuHeader::Colours(vec![]),
        OsuHeader::HitObjects(vec![])
    ];
    let mut iter = collect.iter();
    let mut attributes: Vec<String> = vec![];
    let mut attr_index = 0;
    let mut header_type = "".to_string();
    while let Some(line) = iter.next() {
        println!("attributes: {:?}", attributes);
        if attr_index > 0 {
            match header_type.as_str() {
                "[General]" => {
                    headers[attr_index-1 as usize] = OsuHeader::General(attributes.clone());
                    
                },
                "[Editor]" => {
                    headers[attr_index-1 as usize] = OsuHeader::Editor(attributes.clone());
                },
                "[Metadata]" => {
                    headers[attr_index-1 as usize] = OsuHeader::Metadata(attributes.clone());
                },
                "[Difficulty]" => {
                    headers[attr_index-1 as usize] = OsuHeader::Difficulty(attributes.clone());
                },
                "[Events]" => {
                    headers[attr_index-1 as usize] = OsuHeader::Events(attributes.clone());
                },
                "[TimingPoints]" => {
                    headers[attr_index-1 as usize] = OsuHeader::TimingPoints(attributes.clone());
                },
                "[Colours]" => {
                    headers[attr_index-1 as usize] = OsuHeader::Colours(attributes.clone());
                },
                "[HitObjects]" => {
                    headers[attr_index-1 as usize] = OsuHeader::HitObjects(attributes.clone());
                },
                _ => (),
            }
        }
        attr_index += 1;
        attributes.clear();
        header_type = "".to_string();
        

        for i in line.split("\r\n") {
            if i.contains("osu file format") {
                attr_index = 0;
                break;
            }
            if i.contains("[") {
                header_type = i.to_string();
                println!("HEADER TYPE: {}", header_type);
                continue;
            }
            attributes.push(i.to_string());
            
        }
    }
    headers
}

impl OsuParser {
    pub fn new(file: String) -> Self {
        let headers = parse_headers(file.clone());
        return OsuParser {
            file,
            general: headers[0].clone(),
            editor: headers[1].clone(),
            metadata: headers[2].clone(),
            difficulty: headers[3].clone(),
            events: headers[4].clone(),
            timing_points: headers[5].clone(),
            colours: headers[6].clone(),
            hit_objects: headers[7].clone(),
        }
    }

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
        // verify file is osu!std file
        let general_data = match &self.general {
            OsuHeader::General(data) => data,
            _ => panic!("Invalid header type"),
        };
        println!("GENERAL DATA:\n{:?}", general_data);
        let file_check = self.check_std(general_data);
        match file_check.0 {
            false => panic!("Could not configure ITG file: {}", file_check.1),
            true => (),
        }
        
        const OSU_FIELDS: [&str; 3] = ["Title", "AudioFilename", "PreviewTime"];

        // let mut file = File::create(file).expect("Unable to create file");
        let binding = file.to_string();
        let path = Path::new(&binding);
        let display = path.display();

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };
        file.write(b"#CREDIT:osu2itg;\n#SELECTABLE:YES;\n").expect("Unable to write data");
        self.write_general(&mut file);
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
                        if key == "Title" {
                            let osu_field = OsuTitle { name: PathBuf::from(value) };
                            let sm5_title: SM5Title = From::from(osu_field);
                            file.write(sm5_title.deserialize().as_bytes()).expect("Unable to write data");
                        }
                    }
                }
            }

        }

        let bpm = self.calc_bpm(osu_data);
        file.write(format!("#BPMS:0.000:{:.3};\n#DISPLAYBPM:{:.3};\n", bpm, bpm).as_bytes()).expect("Unable to write data");
        // file.write_all(b"").expect("Unable to write data");
    }

    fn write_general(&mut self, file: &mut File) {
        let general = match &self.general {
            OsuHeader::General(data) => data,
            _ => panic!("Invalid header type"),
        };
        const OSU_FIELDS: [&str; 2] = ["AudioFilename", "PreviewTime"];

        for j in general.iter() {
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
                    else if key == "PreviewTime" {
                        let time = value.parse::<u32>().unwrap();
                        let osu_field = OsuPreviewTime { time };
                        let sm5_preview_time: SM5PreviewTime = From::from(osu_field);
                        file.write(sm5_preview_time.deserialize().as_bytes()).expect("Unable to write data");
                    }
                }
            }
        }
    }
    
    // Checks if file is osu!std file
    fn check_std(&self, data: &Vec<String>) -> (bool, &str) {
        let mut iter = data.iter();
        while let Some(line) = iter.next() {
            if line.contains("Mode") {
                // Check if mode is 0
                let mode = line.split(":").collect::<Vec<&str>>()[1].trim();
                println!("MODE:{}", mode);
                if mode == "0" {
                    return (true, "");
                }
                return (false, "File passed is not osu!std file");
            }
        }
        return (false, "Cannot determine if file is osu!std file");
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

    // Calculate BPM from osu Timing Points
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