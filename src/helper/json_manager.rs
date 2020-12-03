use crate::helper::guid;
use crate::helper::timestamp;
use json::{JsonResult, JsonValue};

pub fn get_guid() -> String{
    return guid::create_random_guid();
}

pub fn get_timestamp() -> String{
    return timestamp::get_time_us();
}

pub fn deserialize(sequence: &str) -> JsonResult<JsonValue>{
    return json::parse(sequence);
}

pub fn serialize() -> String{
    return guid::create_random_guid();
}

