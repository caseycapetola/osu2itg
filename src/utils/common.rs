// Common utility functions

use std::collections::VecDeque;

use num::Integer;
use crate::osu::hit_object::{HitObject};

// Calculate BPM from quarter note length (in milliseconds)
// https://tuneform.com/tools/time-tempo-bpm-to-milliseconds(ms)
// BPM = 60000 / quarter note length (ms)
pub fn calc_bpm(beat_length: f32) -> f32 {
    if beat_length <= 0.0 {
        return 0.0;
    }
    60000.0 / beat_length
}

// Calculate quarter note duration (in milliseconds) from BPM
// 240000.0/(bpm*beat_division as f32)
// 4*(60000.0)/(bpm*beat_division as f32) --> Gives standard beat length depending on bpm and beat division
// i.e. for 120 bpm and beat division of 4, gives 500 ms per beat; for 120 bpm and beat division of 8, gives 250 ms per beat
pub fn calc_beat_duration(bpm: f32, beat_division: i32) -> f32 {
    if bpm <= 0.0 {
        return 0.0;
    }
    240000.0 / (bpm * beat_division as f32)
}

// Returns the minimum beat division required to represent all hit objects accurately for a single BPM chart
// Legacy implementation for single BPM charts
pub fn _get_min_beat_division(hit_objects: &Vec<HitObject>, bpm: f32) -> i32 {
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

// Returns the minimum beat division required to represent all hit objects and timing points accurately
pub fn get_min_beat_division_all(hit_objects: &Vec<HitObject>, bpms: &Vec<(f32, f32)>) -> i32 {
    let mut prev_note_time: f32 = 0.0;
    let manual_offset: f32 = 2.0; // To avoid floating point errors

    if bpms.is_empty() {
        return 4; // Default to quarter notes if no BPM info
    }
    let mut bpm_queue: VecDeque<(f32, f32)> = VecDeque::from(bpms.clone());
    let (_, mut curr_bpm) = bpm_queue.pop_front().unwrap();
    

    let mut note_types: Vec<i32> = Vec::new();

    for (i, obj) in hit_objects.iter().enumerate() {
        if i == 0 {
            prev_note_time = obj.time as f32;
            continue;
        }

        // If we've passed the next BPM change, if it exists, update current BPM and quarter note duration
        while !bpm_queue.is_empty() && obj.time as f32 >= bpm_queue.front().unwrap().0 {
            let (_, next_bpm) = bpm_queue.pop_front().unwrap();
            // curr_bpm_time = next_bpm_time; // Is this needed? --> We probably want to compare directly with deque front
            curr_bpm = next_bpm;
            prev_note_time = obj.time as f32; // Reset previous note time to current note time on BPM change
        }
        
        // If obj.time matches previous note time (can happen on BPM changes), skip to next object
        // This may need to be updated if beat division is inaccurate
        if obj.time as f32 == prev_note_time {
            continue;
        }

        let qn_duration = calc_beat_duration(curr_bpm, 4); // Update quarter note duration based on current BPM

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

// Snap a beat to the nearest interval (8th note = 0.500, 16th note = 0.250, etc.)
pub fn snap_beat_to_interval(beat: f32, interval: f32) -> f32 {
    if interval <= 0.0 {
        return beat;
    }
    (beat / interval).round() * interval
}

// Placement of object or bpm change in musical score
#[derive(Debug, Clone)]
pub struct ScoreObject {
    pub measure_number: i32,
    pub beat_within_measure: i32,
    pub is_hit_object: bool,
    pub hit_object_type: Option<HitObject>,
}
impl ScoreObject {
    pub fn new(measure_number: i32, beat_within_measure: i32, is_hit_object: bool) -> Self {
        Self {
            measure_number,
            beat_within_measure,
            is_hit_object,
            hit_object_type: None,
        }
    }
}
