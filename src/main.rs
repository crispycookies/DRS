mod comm;
mod helper;

use std::{thread, time};
use crate::helper::json_manager::{Node, Message, Protocol};
use std::borrow::Borrow;

fn test1() -> () {
    let mut test = comm::comm::EasyComm {comm : comm::comm_low_level::Comm::default()};
    test.comm.foreign_addr = "127.0.0.1:1880".to_string();
    test.comm.own_addr = "127.0.0.1:1880".to_string();
    test.init(100,true);


    for _ in 0..1000 {
        let n: Node = Node {address: "Hallo".to_string(), priority: 0x3};
        let m: Message = Message{msg_type: 0x1, payload: "Seas".to_string()};
        let p: Protocol = Protocol { id:"Seas".to_string(), timestamp:"Seas".to_string(), node: n, msg: m };

        match test.send_package(p) {
            Ok(_) => { print!("Succeeded in Sending\n") }
            Err(_) => { print!("Failed to send\n") }
        }
        match test.comm.receive() {
            Ok(e) => { print!("Received: {}\n", e) }
            Err(_) => { print!("Failed to send\n") }
        }
    }
}

fn test2() -> () {
    let mut test = comm::comm_low_level::Comm::default();
    test.foreign_addr = "127.0.0.1:1880".to_string();
    test.own_addr = "127.0.0.1:1881".to_string();
    test.make_socket(100, true);
    test.connect_socket();

    for _ in 0..1000 {
        match test.receive() {
            Ok(e) => { print!("Received: {}", e) }
            Err(_) => { print!("Received nothing\n") }
        }
    }
}

fn main() {
    test1();
}
