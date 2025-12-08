pub fn get_hit_object_vec_from_data(data: String) -> Vec<HitObject> {
    let mut hit_objects = Vec::new();
    for line in data.lines() {
        // Check for header value
        if line.contains("[") && !line.contains("HitObjects") {
            println!("Issue with parsing HitObjects section, exiting parse.");
            break;
        }
        if line.trim().is_empty() || line.contains("[") || line.starts_with("//") {
            continue;
        }
        hit_objects.push(HitObject::new(line.to_string()));
    }
    hit_objects
}

#[derive(Debug, Clone)]
pub struct HitObject {
    pub x: i32,
    pub y: i32,
    pub time: i32,
    pub object_type: i32,
    pub hit_sound: i32,
    pub extras: Vec<String>,
    pub is_slider: bool,

    // MAY HAVE TO ADD SLIDER-SPECIFIC FIELDS LATER
}

impl HitObject {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            time: 0,
            object_type: 0,
            hit_sound: 0,
            extras: Vec::new(),
            is_slider: false,
        }
    }
    
    pub fn new(line: String) -> Self {
        let mut hit_object = HitObject::default();

        // Check if object is a slider
        if line.contains("|") {
            hit_object.is_slider = true;
        }
        
        // Parse key-value pairs
        let parts: Vec<&str> = line.splitn(6, ',').collect();
        if parts.len() < 5 {
            println!("Invalid hit object line, skipping.");
            return hit_object;
        }
        hit_object.x = parts[0].trim().parse().unwrap_or(0);
        hit_object.y = parts[1].trim().parse().unwrap_or(0);
        hit_object.time = parts[2].trim().parse().unwrap_or(0);
        hit_object.object_type = parts[3].trim().parse().unwrap_or(0);
        hit_object.hit_sound = parts[4].trim().parse().unwrap_or(0);
        if parts.len() > 5 {
            hit_object.extras = parts[5..].iter().map(|s| s.trim().to_string()).collect();
        }
        
        return hit_object;
    }
}