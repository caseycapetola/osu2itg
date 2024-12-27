use std::ops;

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

pub struct OsuDFA {
    state: NoteState,
    bpm: f32,
    quarter_note: f32,
}

impl ops::Add<OsuDFA> for OsuDFA {
    type Output = OsuDFA;

    fn add(self, other: OsuDFA) -> OsuDFA {
        OsuDFA {
            state: match self.state {
                // NoteState::Quarter(prev_note) => match other.state {
                //         // NoteState::Quarter(_) => NoteState::Quarter(0),
                //         NoteState::Eighth => NoteState::Eighth,
                //         NoteState::Sixteenth => NoteState::Sixteenth,
                //         NoteState::ThirtySecond => NoteState::ThirtySecond,
                //         NoteState::SixtyFourth => NoteState::SixtyFourth,
                //         NoteState::Twelfth => NoteState::Twelfth,
                //         NoteState::TwentyFourth => NoteState::TwentyFourth,
                //         NoteState::FourtyEighth => NoteState::FourtyEighth,
                // },
                NoteState::Eighth => match other.state {
                    // NoteState::Quarter => NoteState::Eighth,
                    NoteState::Eighth => NoteState::Eighth,
                    NoteState::Sixteenth => NoteState::Sixteenth,
                    NoteState::ThirtySecond => NoteState::ThirtySecond,
                    NoteState::SixtyFourth => NoteState::SixtyFourth,
                    NoteState::Twelfth => NoteState::TwentyFourth,
                    NoteState::TwentyFourth => NoteState::Twelfth,
                    NoteState::FourtyEighth => NoteState::FourtyEighth,
                },
                NoteState::Sixteenth => match other.state {
                    // NoteState::Quarter => NoteState::Sixteenth,
                    NoteState::Eighth => NoteState::Sixteenth,
                    NoteState::Sixteenth => NoteState::Sixteenth,
                    NoteState::ThirtySecond => NoteState::ThirtySecond,
                    NoteState::SixtyFourth => NoteState::SixtyFourth,
                    NoteState::Twelfth => NoteState::TwentyFourth,
                    NoteState::TwentyFourth => NoteState::Twelfth,
                    NoteState::FourtyEighth => NoteState::FourtyEighth,
                },
                NoteState::ThirtySecond => NoteState::ThirtySecond,
                NoteState::SixtyFourth => NoteState::SixtyFourth,
                NoteState::Twelfth => NoteState::Twelfth,
                NoteState::TwentyFourth => NoteState::TwentyFourth,
                NoteState::FourtyEighth => NoteState::FourtyEighth,
            },
            bpm: self.bpm + other.bpm,
            quarter_note: self.quarter_note + other.quarter_note,
        }
    }
}

impl OsuDFA {
    pub fn _new(bpm: f32) -> OsuDFA {
        OsuDFA {
            state: NoteState::Eighth,
            bpm,
            quarter_note: 60000.0 / bpm
        }
    }

    pub fn _update(&mut self, distance: f32) -> Result<(), &'static str> {
        let mut new_state = self.state;
        let mut new_distance = distance;

        while new_distance >= self.quarter_note {
            new_distance -= self.quarter_note;
        }
        
        let _note_placement = self._get_distance(new_distance).ok_or("Error: get_distance returned None")?;

        new_state = match new_state {
            // NoteState::Quarter => NoteState::Eighth,
            NoteState::Eighth => NoteState::Sixteenth,
            NoteState::Sixteenth => NoteState::ThirtySecond,
            NoteState::ThirtySecond => NoteState::SixtyFourth,
            NoteState::SixtyFourth => NoteState::SixtyFourth,
            NoteState::Twelfth => todo!(),
            NoteState::TwentyFourth => todo!(),
            NoteState::FourtyEighth => todo!(),
        };

        self.state = new_state;
        Ok(())
        
    }

    fn _get_state(&self) -> NoteState {
        self.state
    }

    fn _get_distance(&self, distance: f32) -> Option<NoteState> {
        if (distance - self.quarter_note).abs() < 1.0 {
            return Some(NoteState::Eighth);
        }
        else if (distance - self.quarter_note / 2.0).abs() < 1.0 {
            return Some(NoteState::Eighth);
        }
        else if (distance - self.quarter_note / 4.0).abs() < 1.0 {
            return Some(NoteState::Sixteenth);
        }
        else if (distance - self.quarter_note / 8.0).abs() < 1.0 {
            return Some(NoteState::ThirtySecond);
        }
        else if (distance - self.quarter_note / 16.0).abs() < 1.0 {
            return Some(NoteState::SixtyFourth);
        }
        else if (distance - self.quarter_note / 3.0).abs() < 1.0 {
            return Some(NoteState::Twelfth);
        }
        else if (distance - self.quarter_note / 6.0).abs() < 1.0 {
            return Some(NoteState::TwentyFourth);
        }
        else {
            return None;
        }

    }
}