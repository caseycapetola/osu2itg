use rand::Rng;
use crate::constants::*;

// Calculate quarter note duration
pub fn calc_qn_duration(bpm: f32) -> f32 {
    60000.0 / bpm
}

enum OsuMode {
    Standard,
    _Taiko,
    _Catch,
    _Mania,
}

impl OsuMode {
    pub fn val(&self) -> i32 {
        match self {
            OsuMode::Standard => 0,
            OsuMode::_Taiko => 1,
            OsuMode::_Catch => 2,
            OsuMode::_Mania => 3,
        }
    }
}

// Checks if file is osu!std file
pub fn check_std(data: &Vec<String>) -> (bool, &str) {
    let mut iter = data.iter();
    while let Some(line) = iter.next() {
        if line.contains("Mode") {
            // Check if mode is 0
            let mode = line.split(":").collect::<Vec<&str>>()[1].trim().parse::<i32>().unwrap();
            if mode == OsuMode::Standard.val() {
                return (true, "");
            }
            return (false, "File passed is not osu!std file");
        }
    }
    return (false, "Cannot determine if file is osu!std file");
}

// TODO: Add flag to toggle on footswitches/crossovers
// Determine next step location -> 0 = left, 1 = right
pub fn next_step(prev: String, new_foot: i8, prev_note_type: i32, note_type: i32) -> String {
    // CASE 1: Previous note was a tap note
    if prev_note_type & OsuNoteType::Tap.val() == OsuNoteType::Tap.val() {
        match prev.as_str() {
            SM5NoteType::LSTEP | SM5NoteType::LSTEP_DRELEASE | SM5NoteType::LSTEP_URELEASE | SM5NoteType::LSTEP_RRELEASE => {
                if new_foot == Foot::RIGHT {
                    let num = rand::thread_rng().gen_range(0..3);
                    if num == 0 {
                        if note_type & 0b10 == 0b10 {
                            return SM5NoteType::DHOLD.to_string();
                        }
                        return SM5NoteType::DSTEP.to_string();
                    }
                    else if num == 1 {
                        if note_type & 0b10 == 0b10 {
                            return SM5NoteType::UHOLD.to_string();
                        }
                        return SM5NoteType::USTEP.to_string();
                    }
                    else {
                        if note_type & 0b10 == 0b10 {
                            return SM5NoteType::RHOLD.to_string();
                        }
                        return SM5NoteType::RSTEP.to_string();
                    }
                } else {
                    if note_type & 0b10 == 0b10 {
                        return "0020".to_string();
                    }
                    return "0010".to_string();
                }
            },
            "0001" | "3001" | "0301" | "0031" => {
                if new_foot == Foot::LEFT {
                    let num = rand::thread_rng().gen_range(0..3);
                    if num == 0 {
                        if note_type & 0b10 == 0b10 {
                            return "0200".to_string();
                        }
                        return "0100".to_string();
                    }
                    else if num == 1 {
                        if note_type & 0b10 == 0b10 {
                            return "0020".to_string();
                        }
                        return "0010".to_string();
                    }
                    else {
                        if note_type & 0b10 == 0b10 {
                            return "2000".to_string();
                        }
                        return "1000".to_string();
                    }
                } else {
                    let num = rand::thread_rng().gen_range(0..2);
                    if num == 0{
                        if note_type & 0b10 == 0b10 {
                            return "0200".to_string();
                        }
                        return "0100".to_string();
                    }
                    if note_type & 0b10 == 0b10 {
                        return "0020".to_string();
                    }
                    return "0010".to_string();
                }
            },
            "0100" | "3100" | "0130" | "0103" => {
                let num = rand::thread_rng().gen_range(0..2);
                if new_foot == 1 {
                    if num == 0 {
                        if note_type & 0b10 == 0b10 {
                            return "0020".to_string();
                        }
                        return "0010".to_string();
                    }
                    else {
                        if note_type & 0b10 == 0b10 {
                            return "0002".to_string();
                        }
                        return "0001".to_string();
                    }
                }
                else {
                    if num == 0 {
                        if note_type & 0b10 == 0b10 {
                            return "0020".to_string();
                        }
                        return "0010".to_string();
                    }
                    else {
                        if note_type & 0b10 == 0b10 {
                            return "2000".to_string();
                        }
                        return "1000".to_string();
                    }
                }
            },
            "0010" | "3010" | "0310" | "0013" => {
                let num = rand::thread_rng().gen_range(0..2);
                if new_foot == 1 {
                    if num == 0 {
                        if note_type & 0b10 == 0b10 {
                            return "0200".to_string();
                        }
                        return "0100".to_string();
                    }
                    else {
                        if note_type & 0b10 == 0b10 {
                            return "0002".to_string();
                        }
                        return "0001".to_string();
                    }
                }
                else {
                    if num == 0 {
                        if note_type & 0b10 == 0b10 {
                            return "0200".to_string();
                        }
                        return "0100".to_string();
                    }
                    else {
                        if note_type & 0b10 == 0b10 {
                            return "2000".to_string();
                        }
                        return "1000".to_string();
                    }
                }
            },
            _ => {
                return "1111".to_string();
            }
        }
    }
    // CASE 2: Previous note was a slider
    else {
        match prev.as_str() {
            "2000" | "2300" | "2030" | "2003" => {
                if new_foot == 1 {
                    let num = rand::thread_rng().gen_range(0..3);
                    if num == 0 {
                        if note_type & 0b10 == 0b10 {
                            return "3200".to_string();
                        }
                        return "3100".to_string();
                    }
                    else if num == 1 {
                        if note_type & 0b10 == 0b10 {
                            return "3020".to_string();
                        }
                        return "3010".to_string();
                    }
                    else {
                        if note_type & 0b10 == 0b10 {
                            return "3002".to_string();
                        }
                        return "3001".to_string();
                    }
                } else {
                    if note_type & 0b10 == 0b10 {
                        return "3020".to_string();
                    }
                    return "3010".to_string();
                }
            },
            "0002" | "3002" | "0302" | "0032" => {
                if new_foot == 1 {
                    let num = rand::thread_rng().gen_range(0..3);
                    if num == 0 {
                        if note_type & 0b10 == 0b10 {
                            return "0203".to_string();
                        }
                        return "0103".to_string();
                    }
                    else if num == 1 {
                        if note_type & 0b10 == 0b10 {
                            return "0023".to_string();
                        }
                        return "0013".to_string();
                    }
                    else {
                        if note_type & 0b10 == 0b10 {
                            return "2003".to_string();
                        }
                        return "1003".to_string();
                    }
                } else {
                    if note_type & 0b10 == 0b10 {
                        return "0023".to_string();
                    }
                    return "0013".to_string();
                }
            },
            "0200" | "3200" | "0230" | "0203" => {
                let num = rand::thread_rng().gen_range(0..2);
                if new_foot == 1 {
                    if num == 0 {
                        if note_type & 0b10 == 0b10 {
                            return "0320".to_string();
                        }
                        return "0310".to_string();
                    }
                    else {
                        if note_type & 0b10 == 0b10 {
                            return "0302".to_string();
                        }
                        return "0301".to_string();
                    }
                }
                else {
                    if num == 0 {
                        if note_type & 0b10 == 0b10 {
                            return "0320".to_string();
                        }
                        return "0310".to_string();
                    }
                    else {
                        if note_type & 0b10 == 0b10 {
                            return "2300".to_string();
                        }
                        return "1300".to_string();
                    }
                }
            },
            "0020" | "3020" | "0320" | "0023" => {
                let num = rand::thread_rng().gen_range(0..2);
                if new_foot == 1 {
                    if num == 0 {
                        if note_type & 0b10 == 0b10 {
                            return "0230".to_string();
                        }
                        return "0130".to_string();
                    }
                    else {
                        if note_type & 0b10 == 0b10 {
                            return "0032".to_string();
                        }
                        return "0031".to_string();
                    }
                }
                else {
                    if num == 0 {
                        if note_type & 0b10 == 0b10 {
                            return "0230".to_string();
                        }
                        return "0130".to_string();
                    }
                    else {
                        if note_type & 0b10 == 0b10 {
                            return "2030".to_string();
                        }
                        return "1030".to_string();
                    }
                }
            },
            _ => {
                return "1111".to_string();
            }
        }
    }
    
}

// Calculate how long (in ms) a given slider is
pub fn _slider_length(length: f32, slider_multiplier: f32, slider_velocity_multiplier: f32, beat_length: f32) -> f32 {
    let mut slider_length = length * slider_multiplier;
    slider_length = slider_length * slider_velocity_multiplier;
    slider_length = slider_length * beat_length;
    return slider_length;
}


pub struct Delimiter;

impl Delimiter {
    pub const _MAC: &'static str = "\n";
    pub const WINDOWS: &'static str = "\r\n";
}

// Returns the type of note in ITG format
fn _note_type_to_itg(note_type: i8) -> i8 {
    note_type & 0b11
}
