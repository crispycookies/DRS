use std::time::{SystemTime, Duration};

struct Time {
    tim_offset : i128
}

impl Time {
    pub fn get_time() -> u128{
        return SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_micros();
    }
    pub fn get_time_with_offset(&self) -> u128{
        return ((Time::get_time() as i128) - self.tim_offset) as u128;
    }
    pub fn calc_offset(rec_time : u128) -> i128{
        return (Time::get_time() - rec_time) as i128;
    }
    pub fn set_auto_offset(&mut self, rec_time : u128) -> i128{
        self.tim_offset = Time::calc_offset(rec_time);
        return self.tim_offset
    }

}
