#[derive(Debug, Clone)]
pub struct Editor {
    pub bookmarks: Vec<i32>,
    pub distance_spacing: f32,
    pub beat_division: i32,
    pub grid_size: i32,
    pub timeline_zoom: f32,
}