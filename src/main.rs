use crate::helper::json_manager::{Node, Protocol, Message};

//mod master;
//mod slave;
mod helper;
mod master;

//use std::thread;

fn main() {
    let n: Node = Node {address: "Hallo".to_string(), priority: 0x3};
    let m: Message = Message{msg_type: 0x1, payload: "Seas".to_string()};
    let p: Protocol = Protocol { id:"Seas".to_string(), timestamp:"Seas".to_string(), node: n, msg: m };

    //println!("{}", helper::json_manager::get_guid());
    //println!("{}", helper::json_manager::get_timestamp());
    //println!("{}", helper::json_manager::serialize(p));
    //helper::json_manager::deserialize("hallo".to_string());
}
