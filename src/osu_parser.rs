// osu!std file parser
use std::{fs::File, io::{self, Read, Write}, path::{Path, PathBuf}, vec}; //, collections::HashMap};
use num::Integer;
use crate::file_tools::{Serialize, OsuArtist, OsuAudioFilename, OsuPreviewTime, OsuTitle, OsuVersion, SM5Artist, SM5AudioFilename, SM5PreviewTime, SM5Title, SM5Version};
use crate::osu_util::{Delimiter, calc_qn_duration, check_std, next_step};
use crate::constants::{Foot, TimingPointFields};

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
    general: OsuHeader,
    _editor: OsuHeader,
    metadata: OsuHeader,
    difficulty: OsuHeader,
    _events: OsuHeader,
    timing_points: OsuHeader,
    _colours: OsuHeader,
    pub hit_objects: OsuHeader,
}


// Parse osu! file headers
fn parse_headers(file: String) -> [OsuHeader; 8] {
    let mut f = File::open(file.clone()).expect("Unable to open file");
    let mut data = String::new();
    f.read_to_string(&mut data).expect("Unable to read data");
    let collect = data.split(&(Delimiter::WINDOWS.to_string() + &Delimiter::WINDOWS.to_string())[..]).map(|s| s.to_string()).collect::<Vec<String>>();
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
        if attr_index > 0 {
            match header_type.as_str() {
                "[General]" => {
                    headers[0] = OsuHeader::General(attributes.clone());
                    
                },
                "[Editor]" => {
                    headers[1] = OsuHeader::Editor(attributes.clone());
                },
                "[Metadata]" => {
                    headers[2] = OsuHeader::Metadata(attributes.clone());
                },
                "[Difficulty]" => {
                    headers[3] = OsuHeader::Difficulty(attributes.clone());
                },
                "[Events]" => {
                    headers[4] = OsuHeader::Events(attributes.clone());
                },
                "[TimingPoints]" => {
                    headers[5] = OsuHeader::TimingPoints(attributes.clone());
                },
                "[Colours]" => {
                    headers[6] = OsuHeader::Colours(attributes.clone());
                },
                "[HitObjects]" => {
                    headers[7] = OsuHeader::HitObjects(attributes.clone());
                },
                _ => (),
            }
        }
        attr_index += 1;
        attributes.clear();
        header_type = "".to_string();
        

        for i in line.split(Delimiter::WINDOWS) {
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

    if !attributes.is_empty() {
        match header_type.as_str() {
            "[General]" => {
                headers[0] = OsuHeader::General(attributes.clone());
                
            },
            "[Editor]" => {
                headers[1] = OsuHeader::Editor(attributes.clone());
            },
            "[Metadata]" => {
                headers[2] = OsuHeader::Metadata(attributes.clone());
            },
            "[Difficulty]" => {
                headers[3] = OsuHeader::Difficulty(attributes.clone());
            },
            "[Events]" => {
                headers[4] = OsuHeader::Events(attributes.clone());
            },
            "[TimingPoints]" => {
                headers[5] = OsuHeader::TimingPoints(attributes.clone());
            },
            "[Colours]" => {
                headers[6] = OsuHeader::Colours(attributes.clone());
            },
            "[HitObjects]" => {
                headers[7] = OsuHeader::HitObjects(attributes.clone());
            },
            _ => (),
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
            _editor: headers[1].clone(),
            metadata: headers[2].clone(),
            difficulty: headers[3].clone(),
            _events: headers[4].clone(),
            timing_points: headers[5].clone(),
            _colours: headers[6].clone(),
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
        let collect = data.split(&(Delimiter::WINDOWS.to_string() + &Delimiter::WINDOWS.to_string())[..]).map(|s| s.to_string()).collect::<Vec<String>>();
        return collect;

    }

    // Write fields to chart file
    pub fn write_chart(&mut self, osu_data: &Vec<String>, file: &str, offset: f32) {
        // verify file is osu!std file
        let general_data = match &self.general {
            OsuHeader::General(data) => data,
            _ => panic!("Invalid header type"),
        };
        let (successful, error_msg) = check_std(general_data);
        match successful {
            false => panic!("Could not configure ITG file: {}", error_msg),
            true => (),
        }

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
        self.write_offset(&mut file, offset);
        let bpm = self.calc_bpm(osu_data);

        let timing_points = match &self.timing_points {
            OsuHeader::TimingPoints(data) => data,
            _ => panic!("Invalid header type"),
        };
        let _bpms = self.get_bpms(&timing_points);
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
                        file.write(sm5_audio_filename.serialize().as_bytes()).expect("Unable to write data");
                    }
                    else if key == "PreviewTime" {
                        let time = value.parse::<u32>().unwrap();
                        let osu_field = OsuPreviewTime { time };
                        let sm5_preview_time: SM5PreviewTime = From::from(osu_field);
                        file.write(sm5_preview_time.serialize().as_bytes()).expect("Unable to write data");
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
                        file.write(sm5_title.serialize().as_bytes()).expect("Unable to write data");
                    }
                    else if key == "ArtistUnicode" {
                        let osu_field = OsuArtist { name: PathBuf::from(value) };
                        let sm5_artist: SM5Artist = From::from(osu_field);
                        file.write(sm5_artist.serialize().as_bytes()).expect("Unable to write data");
                    }
                    else if key == "Version" {
                        let osu_field = OsuVersion { version: PathBuf::from(value) };
                        let sm5_version: SM5Version = From::from(osu_field);
                        file.write(sm5_version.serialize().as_bytes()).expect("Unable to write data");
                    }
                }
            }
        }
    } 

    fn write_offset(&self, file: &mut File, offset: f32) {
        file.write(format!("#OFFSET:{};\n", offset).as_bytes()).expect("Unable to write data");
    }

    // Updated write steps function
    fn write_steps(&self, file: &mut File, bpm: f32) -> io::Result<()> {
        file.write_all("//--------------- dance-single - osu2itg ----------------\n".as_bytes())?;
        file.write_all("#NOTEDATA:;\n#STEPSTYPE:dance-single;\n#DESCRIPTION:;\n#DIFFICULTY:Challenge;\n#METER:727;\n#RADARVALUES:0,0,0,0,0;\n#CREDIT:osu2itg;\n#NOTES:\n".as_bytes())?;
        
        // Get slider multiplier
        let _slider_multiplier = self.get_slider_multiplier();

        // Write one empty measure for buffer
        file.write_all("0000\n0000\n0000\n0000\n,\n".as_bytes()).expect("Unable to write data");
        
        let beat_division = self.get_min_beat_division(bpm);
        
        if let OsuHeader::HitObjects(hit_objects) = &self.hit_objects {

            // Use beat division to determine beat length
            let beat_length = 240000.0/(bpm*beat_division as f32);

            let mut prev_time: f32;
            let mut note_time: f32 = 0.0;
            
            // Track location in measure
            let mut beat_count = 0;

            // Track step location/cadence
            let mut foot: Foot = Foot::new(Foot::LEFT);
            let mut prev_step = "1000".to_string();
            let mut prev_note_type = 0b1;

            for hit_object in hit_objects.iter() {
                // Break apart HitObject and collect the note time
                let parts: Vec<&str> = hit_object.split(',').collect();
                if parts.len() < 4 {
                    continue;
                }
                let note_type = parts[3].parse::<i32>().unwrap();

                // Skip spinners (for now)
                if note_type & 0b1000 == 0b1000 {
                    continue;
                }

                
                prev_time = note_time;
                note_time = parts[2].parse::<f32>().unwrap();

                // Edge Case: First note
                if prev_time == 0.0 {
                    if note_type & 0b10 == 0b10 {
                        file.write_all("2000\n".as_bytes()).expect("Unable to write data");
                        prev_step = "2000".to_string();
                    }
                    else {
                        file.write_all("1000\n".as_bytes()).expect("Unable to write data");
                    }
                    prev_note_type = note_type;
                    foot.switch_foot();
                    beat_count += 1;
                    continue;
                }

                // Calculate number of beats between notes --> adjust constant factor to account for rounding errors
                let mut dist = ((note_time - prev_time + 3.0)/beat_length).floor();
                while dist > 1.0 {
                    if beat_count == beat_division {
                        file.write_all(",\n".as_bytes()).expect("Unable to write data");
                        beat_count = 0;
                    }
                    file.write_all("0000\n".as_bytes()).expect("Unable to write data");
                    beat_count += 1;
                    dist -= 1.0;
                }
                if beat_count == beat_division {
                    file.write_all(",\n".as_bytes()).expect("Unable to write data");
                    beat_count = 0;
                }
                prev_step = next_step(prev_step, foot.state, prev_note_type, note_type);
                file.write_all(prev_step.as_bytes()).expect("Unable to write data");
                file.write_all("\n".as_bytes()).expect("Unable to write data");
                prev_note_type = note_type;
                foot.switch_foot();
                beat_count += 1;
            }
        
            // Complete last measure
            while beat_count < beat_division {
                file.write_all("0000\n".as_bytes()).expect("Unable to write data");
                beat_count += 1;
            }
            file.write_all(";\n".as_bytes()).expect("Unable to write data");
        }
        
        Ok(())
    }


    // Calculate BPM from osu Timing Points
    fn calc_bpm(&self, data: &Vec<String>) -> f32 {
        let mut iter = data.iter();
        let mut bpm = 0.0;
        while let Some(line) = iter.next() {
            if line.contains("[TimingPoints]") {
                let timing_info = line.split(Delimiter::WINDOWS).collect::<Vec<&str>>();
                for i in timing_info.iter() {
                    if i.contains("[") {
                        continue;
                    }
                    let timing_data = i.split(",").collect::<Vec<&str>>();
                    if timing_data[TimingPointFields::UNINHERITED] == "1" {
                        bpm = 60000.0 / timing_data[1].parse::<f32>().unwrap();
                    }
                }
            }
        }
        bpm
    }

    // Determine how many lines to print per measure
    fn get_min_beat_division(&self, bpm: f32) -> i32 {
        if let OsuHeader::HitObjects(hit_objects) = &self.hit_objects {
            let mut note_time = hit_objects
                .first()
                .map(|hit_object| hit_object.split(',').nth(2).unwrap().parse::<f32>().unwrap())
                .unwrap_or(0.0);
            let qn_duration = calc_qn_duration(bpm);
            let mut note_types: Vec<i32> = Vec::new();

            for i in hit_objects {
                let prev_note_time = note_time;
                note_time = i.split(',').nth(2).unwrap_or("-1").parse::<f32>().unwrap();
                if note_time == -1.0 {
                    continue;
                }
                if (note_time - prev_note_time + 2.0)%qn_duration < 4.0 {
                    note_types.push(4);
                }
                else if (note_time - prev_note_time + 2.0)%(qn_duration/2.0) < 4.0 {
                    note_types.push(8);
                }
                else if (note_time - prev_note_time + 2.0)%(qn_duration/3.0) < 4.0 {
                    note_types.push(12);
                }
                else if (note_time - prev_note_time + 2.0)%(qn_duration/4.0) < 4.0 {
                    note_types.push(16);
                }
                else if (note_time - prev_note_time + 2.0)%(qn_duration/6.0) < 4.0 {
                    note_types.push(24);
                }
                else if (note_time - prev_note_time + 2.0)%(qn_duration/8.0) < 4.0 {
                    note_types.push(32);
                }
                else if (note_time - prev_note_time + 2.0)%(qn_duration/12.0) < 4.0 {
                    note_types.push(48);
                }
                else if (note_time - prev_note_time + 2.0)%(qn_duration/16.0) < 4.0 {
                    note_types.push(64);
                }
                else {
                    note_types.push(-1);
                }
            }
            let mut lcm = 1;
            while !note_types.is_empty() {
                let curr_note = note_types.pop().unwrap();
                lcm = lcm.lcm(&curr_note);
            }
            return lcm;
        }
        -1
    }

    fn get_bpms(&self, timing_points: &Vec<String>) -> Vec<(f32, f32)> {
        let mut bpms: Vec<(f32, f32)> = vec![];
        let mut prev_bpm = 0.0;
        let mut prev_time: i32 = 0;
        for i in timing_points.iter() {
            let parts: Vec<&str> = i.split(",").collect();
            if parts.len() < 2 {
                continue;
            }
            if parts[TimingPointFields::UNINHERITED] == "1" {
                // If bpms is empty, add the first bpm
                if bpms.is_empty() {
                    prev_bpm = (60000.0 / parts[TimingPointFields::BEAT_LENGTH].parse::<f32>().unwrap() * 1000.0).round() / 1000.0;
                    bpms.push((0.0, prev_bpm));
                    prev_time = parts[TimingPointFields::TIME].parse::<i32>().unwrap();
                    println!("BPM: {}, Time: {}", prev_bpm, parts[TimingPointFields::TIME]);
                    continue;
                }
                let bpm = (60000.0 / parts[TimingPointFields::BEAT_LENGTH].parse::<f32>().unwrap() * 1000.0).round() / 1000.0;
                let time = parts[TimingPointFields::TIME].parse::<i32>().unwrap();
                let beat_count = ((time - prev_time) as f32 / calc_qn_duration(prev_bpm) * 1000.0).round() / 1000.0;
                bpms.push((beat_count, bpm));
                println!("BPM: {}, Time: {}, Beat Count: {}", bpm, parts[TimingPointFields::TIME], beat_count);
                prev_bpm = bpm;
                prev_time = time;

            }
        }
        bpms
    }

    fn get_slider_multiplier(&self) -> f32 {
        if let OsuHeader::Difficulty(difficulty) = &self.difficulty {
            for i in difficulty {
                let parts: Vec<&str> = i.split(":").collect();
                if parts.len() == 2 {
                    let key = parts[0].trim();
                    let value = parts[1].trim();
                    if key == "SliderMultiplier" {
                        return value.parse::<f32>().unwrap();
                    }
                }
            }
        }
        1.0
    }

    fn prepare_chart(&self, timing_points: &Vec<String>, hit_objects: &Vec<String>) {
        if timing_points.is_empty() || hit_objects.is_empty() {
            return;
        }

        let mut t: usize = 0;
        let mut h: usize = 0;
        // Pop first timing point and hit object
        let mut current_timing_point: &String;
        let mut current_hit_object: &String;

        let mut chart_data: Vec<String> = vec![];
        let mut bpm_changes: Vec<(f32, f32)> = vec![];
        let mut temp_bpm_changes: Vec<(f32, f32)> = vec![];
        let mut hit_objects: Vec<&String> = vec![];
        let mut prev_bpm: f32 = 0.0;
        let mut prev_time: f32 = 0.0;
        let mut prev_timing_point_time: f32 = 0.0;
        let mut prev_hit_object_time: f32 = 0.0;
        let mut foot: Foot = Foot::new(Foot::LEFT);
        let mut prev_step = "1000".to_string();
        let mut prev_note_type = 0b1;

        // Hardcoded to account for worst case
        let beat_division = 192;


        while t < timing_points.len() && h < hit_objects.len() {
            current_timing_point = &timing_points[t];
            current_hit_object = &hit_objects[h];
            
            // Get timing point time
            let timing_point_time = current_timing_point.split(',').nth(0).unwrap().parse::<f32>().unwrap();
            let uninherited = current_timing_point.split(',').nth(6).unwrap().parse::<i32>().unwrap();
            let hit_object_time = current_hit_object.split(',').nth(2).unwrap().parse::<f32>().unwrap();
            let note_type = current_hit_object.split(',').nth(3).unwrap().parse::<i32>().unwrap();

            // Skip timing points that are not uninherited
            if uninherited == 0 {
                t += 1;
                continue;
            }

            // Skip spinners
            if note_type & 0b1000 == 0b1000 {
                h += 1;
                continue;
            }
            
            let bpm = 60000.0 / current_timing_point.split(',').nth(TimingPointFields::BEAT_LENGTH).unwrap().parse::<f32>().unwrap();
            
            if timing_point_time <= hit_object_time {
                if hit_objects.is_empty() {
                    bpm_changes.push((0.0, bpm));
                    prev_timing_point_time = timing_point_time;
                    prev_hit_object_time = timing_point_time;
                    prev_bpm = bpm;
                }
                else {
                    let beat_count = ((timing_point_time - prev_timing_point_time) as f32 / calc_qn_duration(prev_bpm) * 1000.0).round() / 1000.0;
                    bpm_changes.push((beat_count, bpm));
                    temp_bpm_changes.push((beat_count, bpm));
                    prev_timing_point_time = timing_point_time;
                }
            }
            else {
                // CASE 1: No BPM change
                if temp_bpm_changes.is_empty() {
                    if chart_data.is_empty() {
                        // 240,000/bpm = length of whole note/measure --> calculating distance in beats
                        let mut dist = ((hit_object_time - prev_hit_object_time + 3.0)/(240_000.0/(prev_bpm * beat_division as f32))).floor();
                        
                        // Continue with write_steps implementation below
                    }
                }
            }

        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_min_beat_division() {
        let parser = OsuParser::new("assets/REASON/reason_reduced.osu".to_string());
        let bpm = 184.0;
        let result = parser.get_min_beat_division(bpm);
        println!("RESULT: {}", result);
        assert_eq!(result, 16); // Replace with the expected value
        
        // 12th notes case
        let parser2 = OsuParser::new("assets/yomiyori_real/yomiyori.osu".to_string());
        let bpm2 = 220.0;
        let result2 = parser2.get_min_beat_division(bpm2);
        println!("RESULT: {}", result2);
        assert_eq!(result2, 48); // Replace with the expected value
    }
}