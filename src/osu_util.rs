enum _OsuNoteType {
    Tap = 0b01,
    Hold = 0b10,
    Spinner = 0b100,
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