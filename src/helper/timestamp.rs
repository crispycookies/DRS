use std::time::SystemTime;

pub fn get_time_us() -> String {
    return SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_micros().to_string();
}