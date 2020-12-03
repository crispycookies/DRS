use crate::helper::guid;
use crate::helper::timestamp;
use json::{JsonResult, JsonValue};

pub struct Node{
    pub address: String,
    pub priority: u8
}

pub struct Message{
    pub msg_type: u8,
    pub payload: String
}

pub struct Protocol {
    pub id: String,
    pub timestamp: String,
    pub node: Node,
    pub msg: Message
}

pub fn get_guid() -> String{
    return guid::create_random_guid();
}

pub fn get_timestamp() -> String{
    return timestamp::get_time_us();
}

pub fn deserialize(sequence: String) -> JsonResult<JsonValue>{
    return json::parse(&sequence);
}

pub fn serialize(pro: Protocol) -> String{
    return json::stringify(pro)
}

