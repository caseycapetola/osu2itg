use rand::Rng;
use crate::utils::constants::*;

// Checks if file is osu!std file
pub fn check_std_v2(mode: i32) -> (bool, &'static str) {
    if mode == OsuModeV2::STANDARD {
        return (true, "");
    }
    return (false, "File passed is not osu!std file");
}

// TODO: Add flag to toggle on footswitches/crossovers
// Determine next step location -> 0 = left, 1 = right
pub fn next_step(prev: String, new_foot: i8, prev_note_type: i32, note_type: i32) -> String {
    // CASE 1: Previous note was a tap note
    if prev_note_type & OsuNoteTypeV2::TAP == OsuNoteTypeV2::TAP {
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
                        return SM5NoteType::UHOLD.to_string();
                    }
                    return SM5NoteType::USTEP.to_string();
                }
            },
            SM5NoteType::RSTEP | SM5NoteType::RSTEP_LRELEASE | SM5NoteType::RSTEP_DRELEASE | SM5NoteType::RSTEP_URELEASE => {
                if new_foot == Foot::LEFT {
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
                            return SM5NoteType::LHOLD.to_string();
                        }
                        return SM5NoteType::LSTEP.to_string();
                    }
                } else {
                    let num = rand::thread_rng().gen_range(0..2);
                    if num == 0{
                        if note_type & 0b10 == 0b10 {
                            return SM5NoteType::DHOLD.to_string();
                        }
                        return SM5NoteType::DSTEP.to_string();
                    }
                    if note_type & 0b10 == 0b10 {
                        return SM5NoteType::UHOLD.to_string();
                    }
                    return SM5NoteType::USTEP.to_string();
                }
            },
            SM5NoteType::DSTEP | SM5NoteType::DSTEP_LRELEASE | SM5NoteType::DSTEP_URELEASE | SM5NoteType::DSTEP_RRELEASE => {
                let num = rand::thread_rng().gen_range(0..2);
                if new_foot == Foot::RIGHT {
                    if num == 0 {
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
                }
                else {
                    if num == 0 {
                        if note_type & 0b10 == 0b10 {
                            return SM5NoteType::UHOLD.to_string();
                        }
                        return SM5NoteType::USTEP.to_string();
                    }
                    else {
                        if note_type & 0b10 == 0b10 {
                            return SM5NoteType::LHOLD.to_string();
                        }
                        return SM5NoteType::LSTEP.to_string();
                    }
                }
            },
            SM5NoteType::USTEP | SM5NoteType::USTEP_LRELEASE | SM5NoteType::USTEP_DRELEASE | SM5NoteType::USTEP_RRELEASE => {
                let num = rand::thread_rng().gen_range(0..2);
                if new_foot == Foot::RIGHT {
                    if num == 0 {
                        if note_type & 0b10 == 0b10 {
                            return SM5NoteType::DHOLD.to_string();
                        }
                        return SM5NoteType::DSTEP.to_string();
                    }
                    else {
                        if note_type & 0b10 == 0b10 {
                            return SM5NoteType::RHOLD.to_string();
                        }
                        return SM5NoteType::RSTEP.to_string();
                    }
                }
                else {
                    if num == 0 {
                        if note_type & 0b10 == 0b10 {
                            return SM5NoteType::DHOLD.to_string();
                        }
                        return SM5NoteType::DSTEP.to_string();
                    }
                    else {
                        if note_type & 0b10 == 0b10 {
                            return SM5NoteType::LHOLD.to_string();
                        }
                        return SM5NoteType::LSTEP.to_string();
                    }
                }
            },
            _ => {
                return SM5NoteType::QUADSTEP.to_string();
            }
        }
    }
    // CASE 2: Previous note was a slider
    else {
        match prev.as_str() {
            SM5NoteType::LHOLD | SM5NoteType::LHOLD_DRELEASE | SM5NoteType::LHOLD_URELEASE | SM5NoteType::LHOLD_RRELEASE => {
                if new_foot == Foot::RIGHT {
                    let num = rand::thread_rng().gen_range(0..3);
                    if num == 0 {
                        if note_type & OsuNoteTypeV2::SLIDER == OsuNoteTypeV2::SLIDER {
                            return SM5NoteType::DHOLD_LRELEASE.to_string();
                        }
                        return SM5NoteType::DSTEP_LRELEASE.to_string();
                    }
                    else if num == 1 {
                        if note_type & OsuNoteTypeV2::SLIDER == OsuNoteTypeV2::SLIDER {
                            return SM5NoteType::UHOLD_LRELEASE.to_string();
                        }
                        return SM5NoteType::USTEP_LRELEASE.to_string();
                    }
                    else {
                        if note_type & OsuNoteTypeV2::SLIDER == OsuNoteTypeV2::SLIDER {
                            return SM5NoteType::RHOLD_LRELEASE.to_string();
                        }
                        return SM5NoteType::RSTEP_LRELEASE.to_string();
                    }
                } else {
                    if note_type & OsuNoteTypeV2::SLIDER == OsuNoteTypeV2::SLIDER {
                        return SM5NoteType::UHOLD_LRELEASE.to_string();
                    }
                    return SM5NoteType::USTEP_LRELEASE.to_string();
                }
            },
            SM5NoteType::RHOLD | SM5NoteType::RHOLD_LRELEASE | SM5NoteType::RHOLD_DRELEASE | SM5NoteType::RHOLD_URELEASE => {
                if new_foot == 1 {
                    let num = rand::thread_rng().gen_range(0..3);
                    if num == 0 {
                        if note_type & 0b10 == 0b10 {
                            return SM5NoteType::DHOLD_RRELEASE.to_string();
                        }
                        return SM5NoteType::DSTEP_RRELEASE.to_string();
                    }
                    else if num == 1 {
                        if note_type & 0b10 == 0b10 {
                            return SM5NoteType::UHOLD_RRELEASE.to_string();
                        }
                        return SM5NoteType::USTEP_RRELEASE.to_string();
                    }
                    else {
                        if note_type & 0b10 == 0b10 {
                            return SM5NoteType::LHOLD_RRELEASE.to_string();
                        }
                        return SM5NoteType::LSTEP_RRELEASE.to_string();
                    }
                } else {
                    if note_type & 0b10 == 0b10 {
                        return SM5NoteType::UHOLD_RRELEASE.to_string();
                    }
                    return SM5NoteType::USTEP_RRELEASE.to_string();
                }
            },
            SM5NoteType::DHOLD | SM5NoteType::DHOLD_LRELEASE | SM5NoteType::DHOLD_URELEASE | SM5NoteType::DHOLD_RRELEASE => {
                let num = rand::thread_rng().gen_range(0..2);
                if new_foot == 1 {
                    if num == 0 {
                        if note_type & 0b10 == 0b10 {
                            return SM5NoteType::UHOLD_DRELEASE.to_string();
                        }
                        return SM5NoteType::USTEP_DRELEASE.to_string();
                    }
                    else {
                        if note_type & 0b10 == 0b10 {
                            return SM5NoteType::RHOLD_DRELEASE.to_string();
                        }
                        return SM5NoteType::RSTEP_DRELEASE.to_string();
                    }
                }
                else {
                    if num == 0 {
                        if note_type & 0b10 == 0b10 {
                            return SM5NoteType::UHOLD_DRELEASE.to_string();
                        }
                        return SM5NoteType::USTEP_DRELEASE.to_string();
                    }
                    else {
                        if note_type & 0b10 == 0b10 {
                            return SM5NoteType::LHOLD_DRELEASE.to_string();
                        }
                        return SM5NoteType::LSTEP_DRELEASE.to_string();
                    }
                }
            },
            SM5NoteType::UHOLD | SM5NoteType::UHOLD_LRELEASE | SM5NoteType::UHOLD_DRELEASE | SM5NoteType::UHOLD_RRELEASE => {
                let num = rand::thread_rng().gen_range(0..2);
                if new_foot == 1 {
                    if num == 0 {
                        if note_type & 0b10 == 0b10 {
                            return SM5NoteType::DHOLD_URELEASE.to_string();
                        }
                        return SM5NoteType::DSTEP_URELEASE.to_string();
                    }
                    else {
                        if note_type & 0b10 == 0b10 {
                            return SM5NoteType::RHOLD_URELEASE.to_string();
                        }
                        return SM5NoteType::RSTEP_URELEASE.to_string();
                    }
                }
                else {
                    if num == 0 {
                        if note_type & 0b10 == 0b10 {
                            return SM5NoteType::DHOLD_URELEASE.to_string();
                        }
                        return SM5NoteType::DSTEP_URELEASE.to_string();
                    }
                    else {
                        if note_type & 0b10 == 0b10 {
                            return SM5NoteType::LHOLD_URELEASE.to_string();
                        }
                        return SM5NoteType::LSTEP_URELEASE.to_string();
                    }
                }
            },
            _ => {
                return SM5NoteType::QUADSTEP.to_string();
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


// Returns the type of note in ITG format
fn _note_type_to_itg(note_type: i8) -> i8 {
    note_type & 0b11
}
