// osu!std file parser
use std::{fs::File, io::{self, Write}, path::{Path}, vec};
//, collections::HashMap};
use crate::{osu::{colour::get_colours_from_data, events::{Event, get_event_vec_from_data}, hit_object::{HitObject, get_hit_object_vec_from_data}, timing::{TimingPoint, get_timing_point_vec_from_data}}, utils::{common::{_get_min_beat_division, ScoreObject, get_min_beat_division_all, snap_beat_to_interval}, file::parse_file}};
use crate::parser::osu_util::{check_std_v2, next_step};
use crate::utils::common::{calc_bpm, calc_beat_duration};
use crate::utils::constants::{Foot, OsuFields, OsuNoteTypeV2, SM5NoteType};
use crate::osu;

pub struct OsuParserV2 {
    _file: String,
    general: osu::general::General,
    _editor: osu::editor::Editor,
    metadata: osu::metadata::Metadata,
    _difficulty: osu::difficulty::Difficulty,
    events: Vec<osu::events::Event>,
    timing_points: Vec<osu::timing::TimingPoint>,
    _colours: Vec<osu::colour::Colour>,
    pub hit_objects: Vec<osu::hit_object::HitObject>,
}

impl OsuParserV2 {
    pub fn new(file: String) -> Self {
        let file_data = parse_file(file.clone());
        let parser_v2 = OsuParserV2 {
            _file: file,
            general: osu::general::General::new(file_data[OsuFields::GENERAL].clone()),
            _editor: osu::editor::Editor::new(file_data[OsuFields::EDITOR].clone()),
            metadata: osu::metadata::Metadata::new(file_data[OsuFields::METADATA].clone()),
            _difficulty: osu::difficulty::Difficulty::new(file_data[OsuFields::DIFFICULTY].clone()),
            events: get_event_vec_from_data(file_data[OsuFields::EVENTS].clone()),
            timing_points: get_timing_point_vec_from_data(file_data[OsuFields::TIMING_POINTS].clone()),
            _colours: get_colours_from_data(file_data[OsuFields::COLOURS].clone()),
            hit_objects: get_hit_object_vec_from_data(file_data[OsuFields::HIT_OBJECTS].clone()),
        };
        
        parser_v2
    }

    pub fn write_chart(&self, output_path: &str) {
        let (successful, error_msg) = check_std_v2(self.general.mode);
        match successful {
            false => panic!("Could not configure ITG file: {}", error_msg),
            true => (),
        }

        let binding = output_path.to_string();
        let path = Path::new(&binding);
        let display = path.display();

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        file.write(b"#CREDIT:osu2itg;\n#SELECTABLE:YES;\n").expect("Unable to write data");
        self.write_general(&mut file);
        self.write_metadata(&mut file);
        self.write_events(&mut file);
        self.write_offset(&mut file, &self.timing_points);
        let bpms = self.write_bpms(&mut file);

        self.write_steps_v3(&mut file, bpms).expect("Unable to write steps");
        


    }

    fn write_general(&self, file: &mut File) {
        file.write(format!("#MUSIC:{};\n", self.general.audio_filename).as_bytes()).expect("Unable to write data");

        // Convert preview time from milliseconds to seconds
        let preview_start = (self.general.preview_time as f32) / 1000.0;
        file.write(format!("#SAMPLESTART:{};\n", preview_start).as_bytes()).expect("Unable to write data");

        file.write("#SAMPLELENGTH:20.000;\n".as_bytes()).expect("Unable to write data");
    }

    fn write_metadata(&self, file: &mut File) {
        file.write(format!("#TITLE:{};\n", self.metadata.title_unicode).as_bytes()).expect("Unable to write data");
        file.write(format!("#ARTIST:{};\n", self.metadata.artist_unicode).as_bytes()).expect("Unable to write data");
        file.write(format!("#STEPSTITLE:{};\n", self.metadata.version).as_bytes()).expect("Unable to write data");
    }

    // Write events (i.e. background image) to chart file
    fn write_events(&self, file: &mut File) {
        // Write background image
        for event in self.events.iter() {
            if event.event_type == Event::BACKGROUND.to_string() {
                file.write(format!("#BACKGROUND:{};\n", event.event_params[0]).as_bytes()).expect("Unable to write data");
                break;
            }
        }
    }

    // Write offset to chart file, based on osu! file offset
    fn write_offset(&self, file: &mut File, timing_points: &Vec<TimingPoint>) {
        let mut offset: f32 = 0.0;
        for tp in timing_points.iter() {
            if tp.uninherited == true {
                offset = -1.0 * (tp.time as f32 / 1000.0);
                break;
            }
        }
        file.write(format!("#OFFSET:{};\n", offset).as_bytes()).expect("Unable to write data");

    }
    
    // Write BPM changes to chart file, return vector of (time, bpm) tuples
    fn write_bpms(&self, file: &mut File) -> Vec<(f32, f32)> {
        let mut res: Vec<(f32, f32)> = vec![];
        
        let mut bpm_string = String::from("#BPMS:");
        let mut min_bpm = std::f32::MAX;
        let mut max_bpm = std::f32::MIN;

        // Track previous BPM to determine beat counts between changes
        let mut prev_bpm: f32 = 0.0;
        let mut prev_time: f32 = 0.0;
        let mut prev_beat_count: f32 = 0.0;
        let mut prev_beat_duration: f32 = 0.0;
        let manual_offset: f32 = 3.0; // Adjust for rounding errors

        for tp in self.timing_points.iter() {
            if tp.uninherited == true {
                let bpm = calc_bpm(tp.beat_length);
                // let time_in_seconds = tp.time as f32 / 1000.0; // MAY USE THIS LATER, FOR NOW HARDCODE 0.000
                
                if prev_bpm == 0.0 {
                    bpm_string.push_str(&format!("0.000={:.3},", bpm));
                    prev_bpm = bpm;
                    prev_time = tp.time as f32;
                    prev_beat_duration = tp.beat_length
                } else {
                    let beat_count = prev_beat_count + ((tp.time as f32 - prev_time + manual_offset) / prev_beat_duration * 1000.0).round() / 1000.0;
                    
                    // HARDCODED TO 192NDS FOR NOW --> NEED TO CALCULATE MIN BEAT DIVISION TO BE MORE ACCURATE --> This would involve refactoring to print BPMs in write_chart_v3 somewhere
                    // There's a possiblility that we may want to snap BPM changes to nearest beat based on min beat division, but seems to be working for now
                    bpm_string.push_str(&format!("{:.5}={:.3},", snap_beat_to_interval(beat_count, 1.0 / (192.0/4.0)), bpm));
                    
                    prev_bpm = bpm;
                    prev_time = tp.time as f32;
                    prev_beat_count = beat_count;
                    prev_beat_duration = tp.beat_length;
                }
                
                res.push((prev_time, bpm));
                if bpm < min_bpm {
                    min_bpm = bpm;
                }
                if bpm > max_bpm {
                    max_bpm = bpm;
                }
            }
        }
        // Remove trailing comma and add semicolon
        if bpm_string.ends_with(',') {
            bpm_string.pop();
        }
        bpm_string.push(';');
        file.write(bpm_string.as_bytes()).expect("Unable to write data");

        // MAY NOT NEED TO WRITE DISPLAY BPM
        if min_bpm == max_bpm {
            file.write(format!("\n#DISPLAYBPM:{:.3};\n", min_bpm).as_bytes()).expect("Unable to write data");
        } else {
            file.write(format!("\n#DISPLAYBPM:{:.3}:{:.3};\n", min_bpm, max_bpm).as_bytes()).expect("Unable to write data");
        }

        res
    }

    // Legacy implementation of write_steps for single BPM charts
    fn _write_steps(&self, file: &mut File, bpm: f32) -> io::Result<()> {
        // Write standard header
        file.write_all("//--------------- dance-single - osu2itg ----------------\n".as_bytes())?;
        file.write_all("#NOTEDATA:;\n#STEPSTYPE:dance-single;\n#DESCRIPTION:;\n#DIFFICULTY:Challenge;\n#METER:727;\n#RADARVALUES:0,0,0,0,0;\n#CREDIT:osu2itg;\n#NOTES:\n".as_bytes())?;

        // Write one empty measure for buffer
        file.write_all("0000\n0000\n0000\n0000\n,\n".as_bytes()).expect("Unable to write data");

        let beat_division = _get_min_beat_division(&self.hit_objects, bpm);
        println!("Using beat division of {}", beat_division);
        let beat_length = calc_beat_duration(bpm, beat_division);
        println!("Calculated beat length of {}", beat_length);

        let mut prev_time: f32;
        let mut curr_time: f32 = 0.0;

        // Track location in measure
        let mut beat_count = 0;

        // Track step location/cadence
        let mut foot: Foot = Foot::new(Foot::LEFT);
        let mut prev_step  = SM5NoteType::LSTEP.to_string();
        println!("testing: {}", prev_step.as_str() == SM5NoteType::LSTEP);
        let mut prev_note_type = OsuNoteTypeV2::TAP;

        for obj in self.hit_objects.iter() {
            // Skip spinners (for now)
            if obj.object_type & OsuNoteTypeV2::SPINNER != 0 {
                continue;
            }
            prev_time = curr_time;
            curr_time = obj.time as f32;
            let manual_offset = 3.0; // Adjust for rounding errors

            // Edge Case: First note
            if prev_time == 0.0 {
                println!("Object Type: {}", obj.object_type);
                if obj.object_type & OsuNoteTypeV2::SLIDER == OsuNoteTypeV2::SLIDER {
                    println!("First note is a slider");
                    file.write_all(format!("{}\n", SM5NoteType::LHOLD).as_bytes()).expect("Unable to write data");
                    prev_step = SM5NoteType::LHOLD.to_string();
                }
                else {
                    file.write_all(format!("{}\n", SM5NoteType::LSTEP).as_bytes()).expect("Unable to write data");
                }
                prev_note_type = obj.object_type;
                foot.switch_foot();
                println!("First Step: {}", prev_step);
                println!("Foot after switch: {:?}", foot.state);
                beat_count += 1;
                continue;
            }

            let mut dist =  ((curr_time - prev_time + manual_offset)/beat_length).floor();
            while dist > 1.0 {
                if beat_count == beat_division {
                    file.write_all(",\n".as_bytes()).expect("Unable to write data");
                    beat_count = 0;
                }
                file.write_all("0000\n".as_bytes()).expect("Unable to write data");
                beat_count += 1;
                dist -= 1.0;
            }
            // Need to check end of measure if we exit while loop at precise point
            if beat_count == beat_division {
                file.write_all(",\n".as_bytes()).expect("Unable to write data");
                beat_count = 0;
            }
            prev_step = next_step(prev_step, foot.state, prev_note_type, obj.object_type);
            file.write_all(format!("{}\n", prev_step).as_bytes()).expect("Unable to write data");
            prev_note_type = obj.object_type;
            foot.switch_foot();
            beat_count += 1;
        }

        // Complete last measure
        while beat_count < beat_division {
            file.write_all("0000\n".as_bytes()).expect("Unable to write data");
            beat_count += 1;
        }
        file.write_all(";\n".as_bytes()).expect("Unable to write data");


        Ok(())
    }

    fn write_steps_v3(&self, file: &mut File, bpms: Vec<(f32, f32)>) -> io::Result<()> {
        // Get beat division for all BPMs
        let beat_division = get_min_beat_division_all(&self.hit_objects, &bpms.clone());

        // Combine hit objects and BPM changes into a single timeline
        let timeline = build_timeline(&self.hit_objects, &self.timing_points);

        // Check first event is a timing point and set initial time
        let mut init_time: f32;
        // Set initial bpm and beat length
        // let mut current_bpm: f32;
        let mut beat_length: f32;
        if let Some(first_event) = timeline.first() {
            match &first_event.event_type {
                TimelineEventType::BPMChange(tp) => {
                    init_time = first_event.time;
                    // current_bpm = calc_bpm(tp.beat_length);
                    // beat_length = calc_beat_duration(current_bpm, 4);
                    beat_length = tp.beat_length.abs(); // Directly use beat length in ms
                },
                _ => {
                    println!("First event is not a BPM change, cannot proceed.");
                    return Ok(());
                }
            }
        } else {
            println!("Timeline is empty, cannot proceed.");
            return Ok(());
        }


        // Create vector of ScoreObjects to hold processed notes
        let mut score_objects: Vec<ScoreObject> = vec![];

        // Track total beats to account for changes in BPM
        let mut total_beats: f32 = 0.0;

        for event in timeline.iter() {
            match &event.event_type {
                TimelineEventType::BPMChange(tp) => {
                    let beat = total_beats + (event.time - init_time)/beat_length;
                    let measure_number = (beat/4.0).floor() as i32;
                    let beat_in_measure = beat % 4.0;
                    let subdivisions_per_beat = beat_division / 4;
                    let row_placement = (beat_in_measure * subdivisions_per_beat as f32).round() as i32;

                    let score_object = ScoreObject::new(measure_number, row_placement, false);

                    score_objects.push(score_object.clone());

                    // current_bpm = calc_bpm(tp.beat_length);
                    // beat_length = calc_beat_duration(current_bpm, 4);
                    beat_length = tp.beat_length.abs(); // Directly use beat length in ms
                    init_time = event.time;
                    total_beats = (beat * subdivisions_per_beat as f32).round() / subdivisions_per_beat as f32;

                    // println!("BPM Change at time {}: New BPM = {}, Beat Length = {}", event.time, current_bpm, beat_length);
                },
                TimelineEventType::HitObject(obj) => {
                    // println!("Hit Object at time {}: Current BPM = {}, Beat Length = {}", event.time, current_bpm, beat_length);
                    // Process hit object here

                    // Skip spinners (for now)
                    if obj.object_type & OsuNoteTypeV2::SPINNER == OsuNoteTypeV2::SPINNER {
                        continue;
                    }

                    let beat = total_beats + (event.time - init_time)/beat_length;
                    let mut measure_number = (beat/4.0).floor() as i32;
                    let beat_in_measure = beat % 4.0;
                    let subdivisions_per_beat = beat_division / 4;
                    let mut row_placement = (beat_in_measure * subdivisions_per_beat as f32).round() as i32;

                    if row_placement == beat_division {
                        // Edge case where note falls exactly on measure line
                        // Move to next measure
                        measure_number += 1;
                        row_placement = 0;
                    }

                    let mut score_object = ScoreObject::new(measure_number, row_placement, true);
                    score_object.hit_object_type = Some(obj.clone());
                    score_objects.push(score_object.clone());

                    // println!("Hit Object at time {}: Measure {}, Row {}", event.time, measure_number, row_placement);
                },
            }
        }

        // Write steps to file based on score_objects
        file.write_all("//--------------- dance-single - osu2itg ----------------\n".as_bytes())?;
        file.write_all("#NOTEDATA:;\n#STEPSTYPE:dance-single;\n#DESCRIPTION:;\n#DIFFICULTY:Challenge;\n#METER:727;\n#RADARVALUES:0,0,0,0,0;\n#CREDIT:osu2itg;\n#NOTES:\n".as_bytes())?;

        // Iterate through score_objects and write to file
        let mut current_measure = 0;
        let mut beat_count = 0;
        let mut foot: Foot = Foot::new(Foot::LEFT);
        let mut prev_step  = SM5NoteType::LSTEP.to_string();
        let mut prev_note_type = OsuNoteTypeV2::NONE;

        for score_object in score_objects.iter() {
            // Fill in empty measures if needed
            while score_object.measure_number > current_measure {
                while beat_count < beat_division {
                    file.write_all("0000\n".as_bytes()).expect("Unable to write data");
                    beat_count += 1;
                }
                file.write_all(format!(", // Measure {}\n", current_measure).as_bytes()).expect("Unable to write data");
                current_measure += 1;
                beat_count = 0;
            }

            // Fill in empty beats within the measure
            while score_object.beat_within_measure > beat_count {
                file.write_all("0000\n".as_bytes()).expect("Unable to write data");
                beat_count += 1;
            }

            // Write the actual note or BPM change
            if score_object.is_hit_object {
                // Check for initial note
                if prev_note_type == OsuNoteTypeV2::NONE {
                    if let Some(obj) = &score_object.hit_object_type {
                        if obj.object_type & OsuNoteTypeV2::SLIDER == OsuNoteTypeV2::SLIDER {
                            file.write_all(format!("{}\n", SM5NoteType::LHOLD).as_bytes()).expect("Unable to write data");
                            prev_step = SM5NoteType::LHOLD.to_string();
                        } else {
                            file.write_all(format!("{}\n", SM5NoteType::LSTEP).as_bytes()).expect("Unable to write data");
                        }
                        prev_note_type = obj.object_type;
                        foot.switch_foot();
                        beat_count += 1;
                        continue;
                    }
                }

                prev_step = next_step(prev_step, foot.state, prev_note_type, score_object.hit_object_type.as_ref().unwrap().object_type);
                file.write_all(format!("{}\n", prev_step).as_bytes()).expect("Unable to write data");
                prev_note_type = score_object.hit_object_type.as_ref().unwrap().object_type;
                foot.switch_foot();
                beat_count += 1;
            } else {
                // For BPM changes, we can choose to write a special marker or skip
                // Here we choose to skip writing anything
                // 
            }
        }
        // Complete last measure
        while beat_count < beat_division {
            file.write_all("0000\n".as_bytes()).expect("Unable to write data");
            beat_count += 1;
        }
        file.write_all(";\n".as_bytes()).expect("Unable to write data");

        

        Ok(())
    }
}

struct TimelineEvent {
    time: f32,
    event_type: TimelineEventType,
}

// Combine hit objects and BPM changes into a single timeline and sort by time
fn build_timeline(hit_objects: &Vec<HitObject>, timing_points: &Vec<TimingPoint>) -> Vec<TimelineEvent> {
    let mut timeline: Vec<TimelineEvent> = vec![];
        for tp in timing_points.iter() {
            if tp.uninherited == true {
                timeline.push(TimelineEvent {
                    time: tp.time as f32,
                    event_type: TimelineEventType::BPMChange(tp.clone()),
                });
            }
        }
        for obj in hit_objects.iter() {
            timeline.push(TimelineEvent {
                time: obj.time as f32,
                event_type: TimelineEventType::HitObject(obj.clone()),
            });
        }
        // Sort timeline by time --> if tie, prioritize timing points
        timeline.sort_by(|a, b| {
            if a.time == b.time {
                match (&a.event_type, &b.event_type) {
                    (TimelineEventType::BPMChange(_), TimelineEventType::HitObject(_)) => std::cmp::Ordering::Less,
                    (TimelineEventType::HitObject(_), TimelineEventType::BPMChange(_)) => std::cmp::Ordering::Greater,
                    _ => std::cmp::Ordering::Equal,
                }
            } else {
                a.time.partial_cmp(&b.time).unwrap()
            }
        });

    timeline
}

enum TimelineEventType {
    HitObject(HitObject),
    BPMChange(TimingPoint),
}
