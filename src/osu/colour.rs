pub fn get_colours_from_data(data: String) -> Vec<Colour> {
    let mut colours = Vec::new();
    for line in data.lines() {
        if line.contains("[") && !line.contains("Colours") {
            println!("Issue with parsing Colours section, exiting parse.");
            break;
        } else if line.trim().is_empty() || line.contains("[") || line.starts_with("//") {
            continue;
        }
        let parts: Vec<&str> = line.splitn(2, ':').collect();
        if parts.len() != 2 {
            continue;
        }
        let option = parts[0].trim().to_string();
        let colour_values: Vec<&str> = parts[1].trim().split(',').collect();
        if colour_values.len() != 3 {
            continue;
        }
        let red = colour_values[0].trim().parse().unwrap_or(0);
        let green = colour_values[1].trim().parse().unwrap_or(0);
        let blue = colour_values[2].trim().parse().unwrap_or(0);
        colours.push(Colour::new(red, green, blue, option));
    }
    colours
}

#[derive(Debug, Clone)]
pub struct Colour {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub option: String,
}

impl Colour {
    pub fn new(red: u8, green: u8, blue: u8, option: String) -> Self {
        Self { red, green, blue, option }
    }
}