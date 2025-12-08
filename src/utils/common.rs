// Common utility functions

use num::Integer;
use crate::osu::hit_object::{HitObject};

// Calculate BPM from quarter note length (in milliseconds)
// https://tuneform.com/tools/time-tempo-bpm-to-milliseconds-ms
// BPM = 60000 / quarter note length (ms)
pub fn calc_bpm(beat_length: f32) -> f32 {
    if beat_length <= 0.0 {
        return 0.0;
    }
    60000.0 / beat_length
}

// Calculate quarter note duration (in milliseconds) from BPM
pub fn calc_beat_duration(bpm: f32, beat_division: i32) -> f32 {
    if bpm <= 0.0 {
        return 0.0;
    }
    240000.0 / (bpm * beat_division as f32)
}

// 240000.0/(bpm*beat_division as f32)
// 4*(60000.0)/(bpm*beat_division as f32) --> Gives standard beat length depending on bpm and beat division
// i.e. for 120 bpm and beat division of 4, gives 500 ms per beat; for 120 bpm and beat division of 8, gives 250 ms per beat

pub fn get_min_beat_division(hit_objects: &Vec<HitObject>, bpm: f32) -> i32 {
    let mut prev_note_time: f32 = 0.0;
    let manual_offset: f32 = 2.0; // To avoid floating point errors
    let qn_duration = calc_beat_duration(bpm, 4); // Quarter note duration
    let mut note_types: Vec<i32> = Vec::new();

    for (i, obj) in hit_objects.iter().enumerate() {
        if i == 0 {
            prev_note_time = obj.time as f32;
            continue;
        }
        let time_diff = obj.time as f32 - prev_note_time + manual_offset;

        if (time_diff % qn_duration) < 4.0 {
            note_types.push(4); // Quarter note
        } else if (time_diff % (qn_duration / 2.0)) < 4.0 {
            note_types.push(8); // Eighth note
        } else if (time_diff % (qn_duration / 3.0)) < 4.0 {
            note_types.push(12); // Triplet
        } else if (time_diff % (qn_duration / 4.0)) < 4.0 {
            note_types.push(16); // Sixteenth note
        } else if (time_diff % (qn_duration / 6.0)) < 4.0 {
            note_types.push(24); // Sixteenth note triplet
        } else if (time_diff % (qn_duration / 8.0)) < 4.0 {
            note_types.push(32); // Thirty-second note
        } else if (time_diff % (qn_duration / 12.0)) < 4.0 {
            note_types.push(48); // Thirty-second note triplet
        } else if (time_diff % (qn_duration / 16.0)) < 4.0 {
            note_types.push(64); // Sixty-fourth note
        } else if (time_diff % (qn_duration / 24.0)) < 4.0 {
            note_types.push(96); // Sixty-fourth note triplet
        } else if (time_diff % (qn_duration / 32.0)) < 4.0 {
            note_types.push(128); // One hundred twenty-eighth note
        } else {
            note_types.push(192); // Fallback to max division
        }

        prev_note_time = obj.time as f32;
    }
    
    // Take the least common multiple of all note types to get the minimum required beat division
    let mut lcm = 1;
    for &note_type in note_types.iter() {
        lcm = lcm.lcm(&note_type);
    }
    
    lcm
}