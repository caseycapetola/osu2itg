// Calculate quarter note duration
pub fn calc_qn_duration(bpm: f32) -> f32 {
    60000.0 / bpm
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
