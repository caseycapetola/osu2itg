use rand::Rng;

// Calculate quarter note duration
pub fn calc_qn_duration(bpm: f32) -> f32 {
    60000.0 / bpm
}

// TODO: Add flag to toggle on footswitches/crossovers
// Determine next step location -> 0 = left, 1 = right
pub fn next_step(prev: String, new_foot: i8) -> String {
    match prev.as_str() {
        "1000" => {
            if new_foot == 1 {
                let num = rand::thread_rng().gen_range(0..3);
                if num == 0 {
                    return "0100".to_string();
                }
                else if num == 1 {
                    return "0010".to_string();
                }
                else {
                    return "0001".to_string();
                }
            } else {
                return "0010".to_string();
            }
        },
        "0001" => {
            if new_foot == 1 {
                let num = rand::thread_rng().gen_range(0..3);
                if num == 0 {
                    return "0100".to_string();
                }
                else if num == 1 {
                    return "0010".to_string();
                }
                else {
                    return "1000".to_string();
                }
            } else {
                return "0010".to_string();
            }
        },
        "0100" => {
            let num = rand::thread_rng().gen_range(0..2);
            if new_foot == 1 {
                if num == 0 {
                    return "0010".to_string();
                }
                else {
                    return "0001".to_string();
                }
            }
            else {
                if num == 0 {
                    return "0010".to_string();
                }
                else {
                    return "1000".to_string();
                }
            }
        },
        "0010" => {
            let num = rand::thread_rng().gen_range(0..2);
            if new_foot == 1 {
                if num == 0 {
                    return "0100".to_string();
                }
                else {
                    return "0001".to_string();
                }
            }
            else {
                if num == 0 {
                    return "0100".to_string();
                }
                else {
                    return "1000".to_string();
                }
            }
        },
        _ => {
            return "1111".to_string();
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
