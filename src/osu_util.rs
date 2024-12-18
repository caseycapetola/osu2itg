enum OsuNoteType {
    Tap = 0b01,
    Hold = 0b10,
    Spinner = 0b100,
}

// Returns the type of note in ITG format
fn note_type_to_itg(note_type: i8) -> &str {
    note_type & 0b11
}