use crate::utils::constants::TimingPointFields;

pub fn get_timing_point_vec_from_data(data: String) -> Vec<TimingPoint> {
    let mut timing_points = Vec::new();
    for line in data.lines() {
        if line.contains("[") && !line.contains("TimingPoints") {
            println!("Issue with parsing TimingPoints section, exiting parse.");
            break;
        }
        if line.trim().is_empty() || line.contains("[") || line.starts_with("//") {
            continue;
        }
        timing_points.push(TimingPoint::new(line.to_string()));
    }
    timing_points
}

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

impl TimingPoint {
    fn default() -> Self {
        Self {
            time: 0,
            beat_length: 500.0,
            meter: 4,
            sample_set: 1,
            sample_index: 0,
            volume: 100,
            uninherited: false,
            effects: 0,
        }
    }
    
    pub fn new(line: String) -> Self {
        let mut timing_point = TimingPoint::default();
        
        // Check for header value
        if line.contains("[") && !line.contains("TimingPoints") {
            println!("Issue with parsing TimingPoints section, exiting parse.");
        }
        // Parse key-value pairs
        let parts: Vec<&str> = line.splitn(8, ',').collect();
        if parts.len() != 8 {
            println!("Invalid timing point line, skipping.");
            return timing_point;
        }
        
        timing_point.time = parts[TimingPointFields::TIME].trim().parse().unwrap_or(0);
        timing_point.beat_length = parts[TimingPointFields::BEAT_LENGTH].trim().parse().unwrap_or(500.0);
        timing_point.meter = parts[TimingPointFields::METER].trim().parse().unwrap_or(4);
        timing_point.sample_set = parts[TimingPointFields::SAMPLE_SET].trim().parse().unwrap_or(1);
        timing_point.sample_index = parts[TimingPointFields::SAMPLE_INDEX].trim().parse().unwrap_or(0);
        timing_point.volume = parts[TimingPointFields::VOLUME].trim().parse().unwrap_or(100);
        timing_point.uninherited = match parts[TimingPointFields::UNINHERITED].trim().parse::<i32>() {
            Ok(1) => true,
            Ok(0) => false,
            _ => false,
        };
        timing_point.effects = parts[TimingPointFields::EFFECTS].trim().parse().unwrap_or(0);
        
        return timing_point;
    }
}