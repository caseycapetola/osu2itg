#[derive(Debug, Clone)]
pub struct HitObject {
    pub x: i32,
    pub y: i32,
    pub time: i32,
    pub object_type: i32,
    pub hit_sound: i32,
    pub extras: Vec<String>,
}