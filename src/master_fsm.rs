use crate::{comm, helper};
use crate::helper::json_manager::{Node, Message, Protocol};
use std::io::Error;

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum MessageTypes{
    LanBroadcast = 0x1,
    LanBroadcastAck = 0x2,
    BroadcastNewMaster = 0x3,
    BroadcastNewMasterAck = 0x4,
    SendTimestamp = 0x5,
    ResponseWithTime = 0x6,
    TimeOffset = 0x7,
    Invalid = 0x99
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
enum FSM{
    LAN
}

pub struct Master{
    pub communication_if: comm::comm::EasyComm
}

impl Master {


    pub fn run(&mut self, f_addr: String, o_addr : String, timeout : u64) -> () {
        self.communication_if.comm.foreign_addr = f_addr;
        self.communication_if.comm.own_addr = o_addr;
        self.communication_if.init(timeout, true);

        let mut guid = helper::guid::RandomGuid{ guid : "".to_string()};
        guid.create_random_guid();

        let time = helper::time::Time {tim_offset: 0};
        let prio = 0x3;
        let msg_type : MessageTypes = MessageTypes::BroadcastNewMaster;


        loop {
            let cloned_guid  = guid.guid.clone();
            let payload = "hallo".to_string();
            let n: Node = Node { address: self.communication_if.comm.own_addr.clone(), priority: prio };
            let m: Message = Message { msg_type: msg_type as u8, payload };
            let p: Protocol = Protocol { id: cloned_guid, timestamp: time.get_time_with_offset().to_string(), node: n, msg: m };

            match self.communication_if.send_package(p) {
                Ok(_) => {
                    match self.communication_if.receive_package() {
                        Ok(e) => {
                            if e.msg.msg_type == MessageTypes::BroadcastNewMasterAck as u8 {
                                print!("\n---\n");
                                print!("\nTrue\n");
                                print!("\n---\n");
                            }
                        }
                        Err(_) => {
                            print!("Package received is damaged\n")
                        }
                    }


                }
                Err(_) => { print!("Failed to send\n") }
            }
        }
    }
}