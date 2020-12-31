use std::time::{SystemTime};

pub struct Time {
    pub(crate) tim_offset: std::sync::Mutex<i128>
}

impl Time {
    #[allow(dead_code)]
    pub fn get_time(&self) -> u128 {
        return SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_micros();
    }
    #[allow(dead_code)]
    pub fn get_time_with_offset(&self) -> u128 {
        return ((Time::get_time(self) as i128) - *self.tim_offset.lock().unwrap()) as u128;
    }
    #[allow(dead_code)]
    pub fn calc_offset(&self, rec_time: u128) -> i128 {
        let time  = Time::get_time(self) as i128;
        let r_time = rec_time as i128;
        return (time - r_time) as i128;
    }
    #[allow(dead_code)]
    pub fn set_auto_offset(&mut self, rec_time: u128) -> i128 {
        *self.tim_offset.lock().unwrap() = Time::calc_offset(self, rec_time);
        return *self.tim_offset.lock().unwrap();
    }
}
