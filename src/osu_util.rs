use rand::Rng;

// Calculate quarter note duration
pub fn calc_qn_duration(bpm: f32) -> f32 {
    60000.0 / bpm
}

// TODO: Add flag to toggle on footswitches/crossovers
// Determine next step location -> 0 = left, 1 = right
pub fn next_step(prev: String, new_foot: i8, prev_note_type: i32, note_type: i32) -> String {
    if prev_note_type & 0b1 == 0b1 {
        match prev.as_str() {
            "1000" | "1300" | "1030" | "1003" => {
                if new_foot == 1 {
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
                            return "0002".to_string();
                        }
                        return "0001".to_string();
                    }
                } else {
                    if note_type & 0b10 == 0b10 {
                        return "0020".to_string();
                    }
                    return "0010".to_string();
                }
            },
            "0001" | "3001" | "0301" | "0031" => {
                if new_foot == 1 {
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


enum _OsuNoteType {
    Tap = 0b01,
    Hold = 0b10,
    Spinner = 0b100,
}

#[derive(Copy, Clone)]
#[allow(dead_code)] // TODO: Remove when Enum is fully implemented
enum NoteState {
    // Quarter(i8), TODO: --> Do I need to represent quarter notes? I think it is a dead state in DFA.
    Eighth,
    Twelfth,
    Sixteenth,
    TwentyFourth,
    ThirtySecond,
    FourtyEighth,
    SixtyFourth,
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
