pub fn get_event_vec_from_data(data: String) -> Vec<Event> {
    let mut events = Vec::new();
    for line in data.lines() {
        if line.contains("[") && !line.contains("Events") {
            println!("Issue with parsing Events section, exiting parse.");
            break;
        }
        if line.trim().is_empty() || line.contains("[") || line.starts_with("//") {
            continue;
        }
        events.push(Event::new(line.to_string()));
    }
    events
}

#[derive(Debug, Clone)]
pub struct Event {
    pub event_type: String,
    pub start_time: i32,
    pub event_params: Vec<String>,
}

impl Event {
    pub const BACKGROUND: &'static str = "0";

    fn default() -> Self {
        Self {
            event_type: String::new(),
            start_time: 0,
            event_params: Vec::new(),
        }
    }
    
    pub fn new(line: String) -> Self {
        let mut event = Event::default();

        // Check for header value
        if line.contains("[") && !line.contains("Events") {
            println!("Issue with parsing Events section, exiting parse.");
        }
        // Parse key-value pairs
        let parts: Vec<&str> = line.splitn(3, ',').collect();
        if parts.len() < 2 {
            println!("Invalid event line, skipping.");
            return event;
        }
        event.event_type = parts[0].trim().to_string();
        event.start_time = parts[1].trim().parse().unwrap_or(0);
        if parts.len() > 2 {
            event.event_params = parts[2].split(',')
                                        .map(|s| s.trim_start_matches("\"")
                                                        .trim_end_matches("\"")
                                                        .to_string())
                                        .collect();
        }

        event
    }
}