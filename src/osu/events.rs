#[derive(Debug, Clone)]
pub struct Events {
    pub event_type: String,
    pub start_time: i32,
    pub event_params: Vec<String>,
}