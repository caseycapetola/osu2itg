// osu!std file parser
use std::{fs::File, io::{self, stdin, stdout, Read, Write}, path::{Path, PathBuf}, vec}; //, collections::HashMap};
use crate::file_tools::{Deserialize, OsuArtist, OsuAudioFilename, OsuPreviewTime, OsuTitle, OsuVersion, SM5Artist, SM5AudioFilename, SM5PreviewTime, SM5Title, SM5Version};

#[derive(Clone)]
#[derive(Debug)]
pub enum OsuHeader {
    General(Vec<String>),
    // Editor(Vec<String>),
    Metadata(Vec<String>),
    // Difficulty(Vec<String>),
    // Events(Vec<String>),
    // TimingPoints(Vec<String>),
    // Colours(Vec<String>),
    HitObjects(Vec<String>),
}

pub struct OsuParser {
    file: String,
    general: OsuHeader,
    // editor: OsuHeader,
    metadata: OsuHeader,
    // difficulty: OsuHeader,
    // events: OsuHeader,
    // timing_points: OsuHeader,
    // colours: OsuHeader,
    pub hit_objects: OsuHeader,
}

// TODO: Remove function when all headers are implemented
fn temp_parse_headers(file: String) -> [OsuHeader; 3] {
    let mut f = File::open(file.clone()).expect("Unable to open file");
    let mut data = String::new();
    f.read_to_string(&mut data).expect("Unable to read data");
    let collect = data.split("\r\n\r\n").map(|s| s.to_string()).collect::<Vec<String>>();

    let mut headers: [OsuHeader; 3] = [
        OsuHeader::General(vec![]),
        OsuHeader::Metadata(vec![]),
        OsuHeader::HitObjects(vec![]),
    ];
    let mut iter = collect.iter();
    let mut attributes: Vec<String> = vec![];
    let mut attr_index = 0;
    let mut header_type = "".to_string();
    while let Some(line) = iter.next() {
        if attr_index > 0 {
            match header_type.as_str() {
                "[General]" => {
                    headers[0] = OsuHeader::General(attributes.clone());
                },
                "[Metadata]" => {
                    headers[1] = OsuHeader::Metadata(attributes.clone());
                }, 
                "[HitObjects]" => {
                    headers[2] = OsuHeader::HitObjects(attributes.clone());
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
                continue;
            }
            attributes.push(i.to_string());
        }
    }

    // Ensure the last block of data is stored
    if !attributes.is_empty() {
        match header_type.as_str() {
            "[General]" => {
                headers[0] = OsuHeader::General(attributes.clone());
            },
            "[Metadata]" => {
                headers[1] = OsuHeader::Metadata(attributes.clone());
            },
            "[HitObjects]" => {
                headers[2] = OsuHeader::HitObjects(attributes.clone());
            },
            _ => (),
        }
    }
    headers
}

// Parse osu! file headers
// fn parse_headers(file: String) -> [OsuHeader; 8] {
//     let mut f = File::open(file.clone()).expect("Unable to open file");
//     let mut data = String::new();
//     f.read_to_string(&mut data).expect("Unable to read data");
//     let collect = data.split("\r\n\r\n").map(|s| s.to_string()).collect::<Vec<String>>();
//     let mut headers: [OsuHeader; 8] = [
//         OsuHeader::General(vec![]),
//         OsuHeader::Editor(vec![]),
//         OsuHeader::Metadata(vec![]),
//         OsuHeader::Difficulty(vec![]),
//         OsuHeader::Events(vec![]),
//         OsuHeader::TimingPoints(vec![]),
//         OsuHeader::Colours(vec![]),
//         OsuHeader::HitObjects(vec![])
//     ];
//     let mut iter = collect.iter();
//     let mut attributes: Vec<String> = vec![];
//     let mut attr_index = 0;
//     let mut header_type = "".to_string();
//     while let Some(line) = iter.next() {
//         println!("attributes: {:?}", attributes);
//         if attr_index > 0 {
//             match header_type.as_str() {
//                 "[General]" => {
//                     headers[attr_index-1 as usize] = OsuHeader::General(attributes.clone());
                    
//                 },
//                 "[Editor]" => {
//                     headers[attr_index-1 as usize] = OsuHeader::Editor(attributes.clone());
//                 },
//                 "[Metadata]" => {
//                     headers[attr_index-1 as usize] = OsuHeader::Metadata(attributes.clone());
//                 },
//                 "[Difficulty]" => {
//                     headers[attr_index-1 as usize] = OsuHeader::Difficulty(attributes.clone());
//                 },
//                 "[Events]" => {
//                     headers[attr_index-1 as usize] = OsuHeader::Events(attributes.clone());
//                 },
//                 "[TimingPoints]" => {
//                     headers[attr_index-1 as usize] = OsuHeader::TimingPoints(attributes.clone());
//                 },
//                 "[Colours]" => {
//                     headers[attr_index-1 as usize] = OsuHeader::Colours(attributes.clone());
//                 },
//                 "[HitObjects]" => {
//                     headers[attr_index-1 as usize] = OsuHeader::HitObjects(attributes.clone());
//                 },
//                 _ => (),
//             }
//         }
//         attr_index += 1;
//         attributes.clear();
//         header_type = "".to_string();
        

//         for i in line.split("\r\n") {
//             if i.contains("osu file format") {
//                 attr_index = 0;
//                 break;
//             }
//             if i.contains("[") {
//                 header_type = i.to_string();
//                 println!("HEADER TYPE: {}", header_type);
//                 continue;
//             }
//             attributes.push(i.to_string());
            
//         }
//     }
//     headers
// }

impl OsuParser {
    pub fn new(file: String) -> Self {
        let headers = temp_parse_headers(file.clone());
        return OsuParser {
            file,
            // general: headers[0].clone(),
            // editor: headers[1].clone(),
            // metadata: headers[2].clone(),
            // difficulty: headers[3].clone(),
            // events: headers[4].clone(),
            // timing_points: headers[5].clone(),
            // colours: headers[6].clone(),
            // hit_objects: headers[7].clone(),

            general: headers[0].clone(),
            metadata: headers[1].clone(),
            hit_objects: headers[2].clone(),
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
        let file_check = self.check_std(general_data);
        match file_check.0 {
            false => panic!("Could not configure ITG file: {}", file_check.1),
            true => (),
        }
        
        // const OSU_FIELDS: [&str; 3] = ["Title", "AudioFilename", "PreviewTime"];

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
        self.write_metadata(&mut file);
        let _offset = self.write_offset(&mut file);

        let bpm = self.calc_bpm(osu_data);
        file.write(format!("#BPMS:0.000={:.3};\n#DISPLAYBPM:{:.3};\n", bpm, bpm).as_bytes()).expect("Unable to write data");
        self.write_steps(&mut file, bpm).expect("Unable to write steps");
    }

    // Write general fields to chart file
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
                        file.write("#SAMPLELENGTH:20.000;\n".as_bytes()).expect("Unable to write data");
                    }
                }
            }
        }
    }

    // Write metadata fields to chart file
    fn write_metadata(&mut self, file: &mut File) {
        let metadata = match &self.metadata {
            OsuHeader::Metadata(data) => data,
            _ => panic!("Invalid header type"),
        };
        const OSU_FIELDS: [&str; 3] = ["TitleUnicode", "ArtistUnicode", "Version"];

        for j in metadata.iter() {
            // Split key value on ":"
            let parts: Vec<&str> = j.split(":").collect();
            if parts.len() == 2 {
                let key = parts[0].trim();
                let value = parts[1].trim();
                if OSU_FIELDS.contains(&key) {
                    // Process key and value
                    if key == "TitleUnicode" {
                        let osu_field = OsuTitle { name: PathBuf::from(value) };
                        let sm5_title: SM5Title = From::from(osu_field);
                        file.write(sm5_title.deserialize().as_bytes()).expect("Unable to write data");
                    }
                    else if key == "ArtistUnicode" {
                        let osu_field = OsuArtist { name: PathBuf::from(value) };
                        let sm5_artist: SM5Artist = From::from(osu_field);
                        file.write(sm5_artist.deserialize().as_bytes()).expect("Unable to write data");
                    }
                    else if key == "Version" {
                        let osu_field = OsuVersion { version: PathBuf::from(value) };
                        let sm5_version: SM5Version = From::from(osu_field);
                        file.write(sm5_version.deserialize().as_bytes()).expect("Unable to write data");
                    }
                }
            }
        }
    } 


    // Write offset to chart file
    fn write_offset(&self, file: &mut File) -> f32 {
        let mut offset = String::new();
        print!("Enter offset: ");
        let _ = stdout().flush();
        stdin().read_line(&mut offset).expect("Unable to read line");

        let trimmed_offset = offset.trim();

        file.write(format!("#OFFSET:{};\n", trimmed_offset).as_bytes()).expect("Unable to write data");

        match trimmed_offset.parse::<f32>() {
            Ok(value) => value,
            Err(e) => {
                eprintln!("Unable to parse offset: {:?}", e);
                0.0 // Return a default value or handle the error as needed
            }
        }
    }

    // Write steps to chart file
    fn write_steps(&self, file: &mut File, bpm: f32) -> io::Result<()> {
        file.write_all("//--------------- dance-single - osu2itg ----------------\n".as_bytes())?;
        file.write_all("#NOTEDATA:;\n#STEPSTYPE:dance-single;\n#DESCRIPTION:;\n#DIFFICULTY:Challenge;\n#METER:727;\n#RADARVALUES:0,0,0,0,0;\n#CREDIT:osu2itg;\n#NOTES:\n".as_bytes())?;

        let measure_length = (self.calc_qn_duration(bpm) * 4.0).round(); // as i32;
        let quarter_note_duration = self.calc_qn_duration(bpm).round();
        let eighth_note_duration = (quarter_note_duration / 2.0).round();
        let sixteenth_note_duration = (quarter_note_duration / 4.0).round();

        if let OsuHeader::HitObjects(hit_objects) = &self.hit_objects {
            let _measures: Vec<Vec<(String, i32)>> = vec![];
            let mut current_measure: Vec<(f32, i32)> = vec![];

            // Determine the time of the first note
            let first_note_time = hit_objects
                .first()
                .map(|hit_object| hit_object.split(',').nth(2).unwrap().parse::<f32>().unwrap())
                .unwrap_or(0.0);

            let mut current_time = first_note_time; // TODO: Adjust this to align with the start of the measure
            println!("FIRST NOTE TIME: {}", first_note_time);
            println!("MEASURE LENGTH: {}", measure_length);
            let mut prev_time: f32;
            let mut note_time: f32 = 0.0;
            for hit_object in hit_objects.iter() {
                let parts: Vec<&str> = hit_object.split(',').collect();
                prev_time = note_time;
                note_time = parts[2].parse::<f32>().unwrap();
                // println!("TIME: {}", note_time);

                // Check if note is in the same measure
                if note_time  >= current_time {
                    println!("NOTE TIME: {} --- CURRENT TIME: {}", note_time, current_time);
                    // Flush current measure to measures buffer/file
                    let max_value = current_measure
                        .iter()
                        .max_by_key(|&(_, value)| value)
                        .map(|&(_, value)| value)
                        .unwrap_or(0);
                    println!("WRITING...");
                    println!("current_measure: {:?}", current_measure);
                    println!("max_value: {}", max_value);
                    let beat_count = measure_length / max_value as f32;
                    let mut found_tuples: Vec<(f32, i32)> = vec![];
                    for i in 0..max_value {
                        let mut find = false;
                        for (note, _value) in current_measure.iter() {
                            // Check if the tuple is already in the found_tuples vector
                            if found_tuples.iter().any(|&(n, _)| n == *note) {
                                continue;
                            }
                            if *note/beat_count - (i as f32) < 1.0 {
                                println!("FOUND: {}", i);
                                find = true;
                                file.write_all("1000\n".as_bytes()).expect("Unable to write data");
                                
                                // Push a copy of the tuple to the found_tuples vector
                                found_tuples.push((*note, *_value));
                            }
                        }
                        if !find {
                            file.write_all("0000\n".as_bytes()).expect("Unable to write data");
                        }
                    }
                    file.write(",\n".as_bytes()).expect("Unable to write data");
                    current_measure.clear();

                    // Write empty measures
                    let mut empty_count = ((note_time - current_time)/measure_length).trunc() as i32 - 1;
                    current_time += measure_length*empty_count as f32;
                    while empty_count > 0 {
                        file.write("0000\n0000\n0000\n0000\n,\n".as_bytes()).expect("Unable to write data");
                        empty_count -= 1;
                        current_time += measure_length;
                    }
                }

                // If in same measure, write note to current_measure buffer
                // let beat = note_time - first_note_time;
                // println!("BEAT: {}", beat);
                if (note_time-prev_time)%quarter_note_duration < 2.0 || (note_time-prev_time)%quarter_note_duration > quarter_note_duration-2.0 {
                    // println!("QUARTER");
                    current_measure.push(((note_time-first_note_time)%measure_length, 4));
                }
                else if (note_time-prev_time)%eighth_note_duration < 2.0 || (note_time-prev_time)%eighth_note_duration > eighth_note_duration-2.0 {
                    // println!("EIGHTH");
                    current_measure.push(((note_time-first_note_time)%measure_length, 8));
                }
                else if (note_time-prev_time)%sixteenth_note_duration < 2.0 || (note_time-prev_time)%sixteenth_note_duration > sixteenth_note_duration-2.0 {
                    // println!("SIXTEENTH");
                    current_measure.push(((note_time-first_note_time)%measure_length, 16));
                }
                else {
                    // println!("INITIAL");
                    current_measure.push(((note_time-first_note_time)%measure_length, 4));
                }
            }

        }

        Ok(())
    }

    // Checks if file is osu!std file
    fn check_std(&self, data: &Vec<String>) -> (bool, &str) {
        let mut iter = data.iter();
        while let Some(line) = iter.next() {
            if line.contains("Mode") {
                // Check if mode is 0
                let mode = line.split(":").collect::<Vec<&str>>()[1].trim();
                if mode == "0" {
                    return (true, "");
                }
                return (false, "File passed is not osu!std file");
            }
        }
        return (false, "Cannot determine if file is osu!std file");
    }

    // Calculate BPM from osu Timing Points
    fn calc_bpm(&self, data: &Vec<String>) -> f32 {
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

    // Calculate quarter note duration
    fn calc_qn_duration(&self, bpm: f32) -> f32 {
        60000.0 / bpm
    }
}