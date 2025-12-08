#[derive(Debug, Clone)]
pub struct Difficulty {
    pub hp_drain_rate: f32,
    pub circle_size: f32,
    pub overall_difficulty: f32,
    pub approach_rate: f32,
    pub slider_multiplier: f32,
    pub slider_tick_rate: f32,
}

impl Difficulty {
    fn default() -> Self {
        Self {
            hp_drain_rate: 5.0,
            circle_size: 5.0,
            overall_difficulty: 5.0,
            approach_rate: 5.0,
            slider_multiplier: 1.4,
            slider_tick_rate: 1.0,
        }
    }
    
    pub fn new(data: String) -> Self {
        let mut difficulty = Difficulty::default();
        
        for line in data.lines() {
            // Check for header value
            if line.contains("[") && !line.contains("Difficulty") {
                println!("Issue with parsing Difficulty section, exiting parse.");
                break;
            } 
            if line.trim().is_empty() || line.contains("[") || line.starts_with("//") {
                continue;
            }
            // Parse key-value pairs
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() != 2 {
                continue;
            }
            let key = parts[0].trim();
            let value = parts[1].trim();
            match key {
                "HPDrainRate" => difficulty.hp_drain_rate = value.parse().unwrap_or(5.0),
                "CircleSize" => difficulty.circle_size = value.parse().unwrap_or(5.0),
                "OverallDifficulty" => difficulty.overall_difficulty = value.parse().unwrap_or(5.0),
                "ApproachRate" => difficulty.approach_rate = value.parse().unwrap_or(5.0),
                "SliderMultiplier" => difficulty.slider_multiplier = value.parse().unwrap_or(1.4),
                "SliderTickRate" => difficulty.slider_tick_rate = value.parse().unwrap_or(1.0),
                _ => {}
            }
        }
        
        return difficulty;
    }
}