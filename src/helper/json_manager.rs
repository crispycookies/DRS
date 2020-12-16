use crate::helper::guid;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Node{
    pub address: String,
    pub priority: u8
}

#[derive(Serialize, Deserialize)]
pub struct Message{
    pub msg_type: u8,
    pub payload: String
}

#[derive(Serialize, Deserialize)]
pub struct Protocol {
    pub id: String,
    pub timestamp: String,
    pub node: Node,
    pub msg: Message
}

#[allow(dead_code)]
pub fn deserialize(sequence: String) -> std::io::Result<Protocol>{
    match serde_json::from_str::<Protocol>(sequence.as_str()) {
        Ok(e) => {Ok(e)}
        Err(_) => {Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Data parsed is erroneous"))}
    }

}
#[allow(dead_code)]
pub fn serialize(pro: Protocol) -> String {
    return serde_json::to_string(&pro).expect("Could not Serialize Protocol")
}