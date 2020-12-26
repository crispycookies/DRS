use crate::{comm, helper};
use crate::helper::json_manager::{Node, Message, Protocol};
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum MessageTypes {
    LanBroadcast = 0x1,
    LanBroadcastAck = 0x2,
    BroadcastNewMaster = 0x3,
    BroadcastNewMasterAck = 0x4,
    SendTimestamp = 0x5,
    ResponseWithTime = 0x6,
    TimeOffset = 0x7,
    Invalid = 0x99,
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Client {
    url: String
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
enum FSM {
    StartUp,
    LAN,
}


pub struct Master {
    pub communication_if: comm::comm::EasyComm,
    pub time: helper::time::Time,
    pub prio: u8,
    pub client_vector: HashMap<u8, Client>
}

#[allow(dead_code)]
impl Master {
    pub fn get_url(&self, url: String) -> String {
        let pos = url.find(':');
        url[0..pos.expect("Expected Port; none given")].to_string()
    }
    pub fn match_ack(&mut self, p: Protocol, ack_msg_type: MessageTypes) -> Option<Protocol> {
        return match self.communication_if.send_package(p.clone()) {
            Ok(_) => {
                match self.communication_if.receive_package() {
                    Ok(e) => {
                        if e.msg.msg_type == ack_msg_type as u8 {
                            Some(e)
                        } else {
                            None
                        }
                    }

                    Err(_) => {
                        None
                    }
                }
            }
            Err(_) => {
                None
            }
        };
    }
    pub fn broadcast_lan(&mut self, guid: String) -> bool {
        let msg_type: MessageTypes = MessageTypes::LanBroadcast;
        let payload = "".to_string();
        let n: Node = Node { address: self.get_url(self.communication_if.comm.own_addr.clone()), priority: self.prio };
        let m: Message = Message { msg_type: msg_type as u8, payload };
        let p: Protocol = Protocol { id: guid, timestamp: self.time.get_time_with_offset().to_string(), node: n, msg: m };

        return match self.match_ack(p, MessageTypes::LanBroadcastAck) {
            None => {
                print!("None\n");
                false
            }
            Some(_) => {
                print!("---\n");
                print!("True\n");
                print!("---\n");
                true
            }
        };
    }

    pub fn broadcast_new_master(&mut self, guid: String) -> bool {
        let msg_type: MessageTypes = MessageTypes::BroadcastNewMaster;
        let payload = "".to_string();
        let n: Node = Node { address: self.get_url(self.communication_if.comm.own_addr.clone()), priority: self.prio };
        let m: Message = Message { msg_type: msg_type as u8, payload };
        let p: Protocol = Protocol { id: guid, timestamp: self.time.get_time_with_offset().to_string(), node: n, msg: m };

        return match self.match_ack(p, MessageTypes::BroadcastNewMasterAck) {
            None => {
                print!("None\n");
                false
            }
            Some(_) => {
                print!("---\n");
                print!("True\n");
                print!("---\n");
                true
            }
        };
    }
    pub fn broadcast_offset(&mut self, guid: String) -> bool {
        let msg_type: MessageTypes = MessageTypes::LanBroadcast;
        let payload = self.time.tim_offset.to_string();
        let n: Node = Node { address: self.get_url(self.communication_if.comm.own_addr.clone()), priority: self.prio };
        let m: Message = Message { msg_type: msg_type as u8, payload };
        let p: Protocol = Protocol { id: guid, timestamp: self.time.get_time_with_offset().to_string(), node: n, msg: m };

        return match self.match_ack(p, MessageTypes::LanBroadcastAck) {
            None => {
                print!("None\n");
                false
            }
            Some(_) => {
                print!("---\n");
                print!("True\n");
                print!("---\n");
                true
            }
        };
    }


    pub fn run(&mut self, f_addr: String, o_addr: String, timeout: u64) -> () {
        self.communication_if.comm.foreign_addr = f_addr;
        self.communication_if.comm.own_addr = o_addr;
        self.communication_if.init(timeout, true);

        let mut guid = helper::guid::RandomGuid { guid: "".to_string() };
        guid.create_random_guid();

        let _fsm = FSM::StartUp;
        let mut client_vector: HashMap<u8, Client> = HashMap::new();

        loop {
            match self.communication_if.receive_package() {
                Ok(e) => {
                    if e.msg.msg_type == MessageTypes::LanBroadcast as u8 {
                        let msg_type: MessageTypes = MessageTypes::LanBroadcastAck;
                        let payload = "".to_string();
                        let n: Node = Node { address: self.get_url(self.communication_if.comm.own_addr.clone()), priority: self.prio };
                        let m: Message = Message { msg_type: msg_type as u8, payload };
                        let p: Protocol = Protocol { id: guid.guid.clone(), timestamp: self.time.get_time_with_offset().to_string(), node: n, msg: m };


                        let f_addr_recvd = e.node.address.clone();
                        if client_vector.contains_key(&e.node.priority) {
                            print!("Client with Priority {} is already registered for address {}, dropping address {}\n", e.node.priority, client_vector.get(&e.node.priority).unwrap().url, e.node.address);
                        } else {
                            client_vector.insert(e.node.priority, Client { url: e.node.address });
                        }

                        self.communication_if.comm.foreign_addr = f_addr_recvd;

                        match self.communication_if.send_package(p) {
                            Ok(_) => {}
                            Err(_) => {}
                        }
                    } else {
                        if client_vector.is_empty() {
                            print!("---");
                            print!("Nothing Received");
                            print!("---");
                        } else {
                            print!("Clients Connected #{}", client_vector.len());
                        }

                    }

                }

                Err(_) => {}
            }

            //match fsm {
            //    FSM::StartUp => {}
            //    FSM::LAN => {}
            //}
            //let guid_clone = guid.guid.clone();
            //self.broadcast_new_master(guid_clone);
        }
    }
}