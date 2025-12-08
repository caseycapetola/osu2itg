// Common utility functions

// Calculate BPM from quarter note length (in milliseconds)
// https://tuneform.com/tools/time-tempo-bpm-to-milliseconds-ms
// BPM = 60000 / quarter note length (ms)
pub fn calc_bpm(beat_length: f32) -> f32 {
    if beat_length <= 0.0 {
        return 0.0;
    }
    60000.0 / beat_length
}

// 240000.0/(bpm*beat_division as f32)
// 4*(60000.0)/(bpm*beat_division as f32) --> Gives standard beat length depending on bpm and beat division
// i.e. for 120 bpm and beat division of 4, gives 500 ms per beat; for 120 bpm and beat division of 8, gives 250 ms per beat