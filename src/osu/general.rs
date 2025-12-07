#[derive(Debug, Clone)]
pub struct General {
    pub audio_filename: String,
    pub audio_lead_in: i32,
    pub audio_hash: String, // DEPRECATED
    pub preview_time: i32,
    pub countdown: i8,
    pub sample_set: i8,
    pub stack_leniency: f32,
    pub mode: i8,
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