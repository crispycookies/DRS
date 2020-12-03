use crate::helper::guid;
use crate::helper::timestamp;
use json::{JsonResult, JsonValue};

struct Node{
    address: String,
    priority: u8
}

struct Message{
    msg_type: u8,
    payload: String
}

pub struct Protocol {
    id: String,
    timestamp: String,
    node: Node,
    msg: Message
}

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

