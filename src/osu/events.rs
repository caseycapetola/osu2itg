#[derive(Debug, Clone)]
pub struct Events {
    pub event_type: String,
    pub start_time: i32,
    pub event_params: Vec<String>,
}

impl Events {
    fn default() -> Self {
        Self {
            event_type: String::new(),
            start_time: 0,
            event_params: Vec::new(),
        }
    }
    
    pub fn new(data: String) -> Self {
        let mut events = Events::default();
        
        for line in data.lines() {
            // Check for header value
            if line.contains("[") && !line.contains("Events") {
                println!("Issue with parsing Events section, exiting parse.");
                break;
            } else if line.trim().is_empty() || line.contains("[") || line.starts_with("//") {
                continue;
            }
            // Parse key-value pairs
            let parts: Vec<&str> = line.splitn(2, ',').collect();
            if parts.len() < 2 {
                continue;
            }
            events.event_type = parts[0].trim().to_string();
            events.start_time = parts[1].trim().parse().unwrap_or(0);
            if parts.len() > 2 {
                events.event_params = parts[2..].iter().map(|s| s.trim().to_string()).collect();
            }
        }
        
        return events;
    }
}