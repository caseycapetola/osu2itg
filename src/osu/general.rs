#[derive(Debug, Clone)]
pub struct General {
    pub audio_filename: String,
    pub audio_lead_in: i32,
    pub audio_hash: String, // DEPRECATED
    pub preview_time: i32,
    pub countdown: i32,
    pub sample_set: String,
    pub stack_leniency: f32,
    pub mode: i32,
    pub letterbox_in_breaks: bool,
    pub story_fire_in_front: bool, // DEPRECATED
    pub use_skin_sprites: bool,
    pub always_show_playfield: bool, // DEPRECATED
    pub overlay_position: String,
    pub skin_preference: String,
    pub epilepsy_warning: bool,
    pub countdown_offset: i32,
    pub special_style: bool,
    pub widescreen_storyboard: bool,
    pub samples_match_playback_rate: bool,
}

impl General {
    fn default() -> Self {
        Self {
            audio_filename: String::new(),
            audio_lead_in: 0,
            audio_hash: String::new(),
            preview_time: -1,
            countdown: 1,
            sample_set: "Normal".to_string(),
            stack_leniency: 0.7,
            mode: 0,
            letterbox_in_breaks: false,
            story_fire_in_front: true,
            use_skin_sprites: false,
            always_show_playfield: false,
            overlay_position: "NoChange".to_string(),
            skin_preference: String::new(),
            epilepsy_warning: false,
            countdown_offset: 0,
            special_style: false,
            widescreen_storyboard: false,
            samples_match_playback_rate: false,
        }
    }
    
    pub fn new(data: String) -> Self {
        let mut general = General::default();

        for line in data.lines() {
            // Check for header value
            if line.contains("[") && !line.contains("General") {
                println!("Issue with parsing General section, exiting parse.");
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
                "AudioFilename" => general.audio_filename = value.to_string(),
                "AudioLeadIn" => general.audio_lead_in = value.parse().unwrap_or(0),
                "AudioHash" => general.audio_hash = value.to_string(),
                "PreviewTime" => general.preview_time = value.parse().unwrap_or(-1),
                "Countdown" => general.countdown = value.parse().unwrap_or(1),
                "SampleSet" => general.sample_set = if value.is_empty() { "Normal".to_string() } else { value.to_string() },
                "StackLeniency" => general.stack_leniency = value.parse().unwrap_or(0.7),
                "Mode" => general.mode = value.parse().unwrap_or(0),
                "LetterboxInBreaks" => general.letterbox_in_breaks = value.parse().unwrap_or(false),
                "StoryFireInFront" => general.story_fire_in_front = value.parse().unwrap_or(true),
                "UseSkinSprites" => general.use_skin_sprites = value.parse().unwrap_or(false),
                "AlwaysShowPlayfield" => general.always_show_playfield = value.parse().unwrap_or(false),
                "OverlayPosition" => general.overlay_position = value.to_string(),
                "SkinPreference" => general.skin_preference = value.to_string(),
                "EpilepsyWarning" => general.epilepsy_warning = value.parse().unwrap_or(false),
                "CountdownOffset" => general.countdown_offset = value.parse().unwrap_or(0),
                "SpecialStyle" => general.special_style = value.parse().unwrap_or(false),
                "WidescreenStoryboard" => general.widescreen_storyboard = value.parse().unwrap_or(false),
                "SamplesMatchPlaybackRate" => general.samples_match_playback_rate = value.parse().unwrap_or(false),
                _ => {}
            }
        }
        general
    }
}