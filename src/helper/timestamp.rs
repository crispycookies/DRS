use std::time::SystemTime;

#[allow(dead_code)]
pub fn get_time_us() -> String {
    return SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_micros().to_string();
}