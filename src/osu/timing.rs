#[derive(Debug, Clone)]
pub struct TimingPoint {
    pub time: i32,
    pub beat_length: f32,
    pub meter: i32,
    pub sample_set: i32,
    pub sample_index: i32,
    pub volume: i32,
    pub uninherited: bool,
    pub effects: i32,
}