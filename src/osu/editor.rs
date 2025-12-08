#[derive(Debug, Clone)]
pub struct Editor {
    pub bookmarks: Vec<i32>,
    pub distance_spacing: f32,
    pub beat_division: i32,
    pub grid_size: i32,
    pub timeline_zoom: f32,
}

impl Editor {
    fn default() -> Self {
        Self {
            bookmarks: Vec::new(),
            distance_spacing: 1.0,
            beat_division: 4,
            grid_size: 32,
            timeline_zoom: 1.0,
        }
    }
    
    pub fn new(data: String) -> Self {
        let mut editor = Editor::default();
        
        for line in data.lines() {
            // Check for header value
            if line.contains("[") && !line.contains("Editor") {
                println!("Issue with parsing Editor section, exiting parse.");
                break;
            } else if line.trim().is_empty() || line.contains("[") || line.starts_with("//") {
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
                "Bookmarks" => {
                    editor.bookmarks = value
                        .split(',')
                        .filter_map(|s| s.trim().parse().ok())
                        .collect();
                }
                "DistanceSpacing" => editor.distance_spacing = value.parse().unwrap_or(1.0),
                "BeatDivisor" => editor.beat_division = value.parse().unwrap_or(4),
                "GridSize" => editor.grid_size = value.parse().unwrap_or(32),
                "TimelineZoom" => editor.timeline_zoom = value.parse().unwrap_or(1.0),
                _ => {}
            }
        }
        
        return editor;
    }
}