use crate::{comm, helper};
use crate::helper::json_manager::{Node, Message, Protocol};
use std::collections::HashMap;
use std::any::Any;


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

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum Register {
    NoneReceived = 0,
    NoneRegistered,
    OneRegistered,
    FailedAck,
}

pub struct InnerMaster {
    pub communication_if: std::sync::Mutex<comm::comm::EasyComm>,
    pub time: std::sync::Arc<std::sync::Mutex<helper::time::Time>>,
    pub prio: u8,
    pub client_vector: HashMap<u8, Client>,
}

#[derive(Clone)]
pub struct Master {
    pub(crate) inner: std::sync::Arc<std::sync::Mutex<InnerMaster>>
}

#[allow(dead_code)]
impl Master {

    pub fn get_url(&self, url: String) -> String {
        let pos = url.find(':');
        url[0..pos.expect("Expected Port; none given")].to_string()
    }
    pub fn match_ack(&mut self, p: Protocol, ack_msg_type: MessageTypes) -> Option<Protocol> {
        return match self.inner.lock().unwrap().communication_if.lock().unwrap().send_package(p.clone()) {
            Ok(_) => {
                match self.inner.lock().unwrap().communication_if.lock().unwrap().receive_package() {
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
        let n: Node = Node { address: self.get_url(self.inner.lock().unwrap().communication_if.lock().unwrap().comm.own_addr.clone()), priority: self.inner.lock().unwrap().prio };
        let m: Message = Message { msg_type: msg_type as u8, payload };
        let p: Protocol = Protocol { id: guid, timestamp: self.inner.lock().unwrap().time.lock().unwrap().get_time_with_offset().to_string(), node: n, msg: m };

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
        let n: Node = Node { address: self.get_url(self.inner.lock().unwrap().communication_if.lock().unwrap().comm.own_addr.clone()), priority: self.inner.lock().unwrap().prio };
        let m: Message = Message { msg_type: msg_type as u8, payload };
        let p: Protocol = Protocol { id: guid, timestamp: self.inner.lock().unwrap().time.lock().unwrap().get_time_with_offset().to_string(), node: n, msg: m };

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
        let payload = (*self.inner.lock().unwrap().time.lock().unwrap().tim_offset.lock().unwrap()).to_string();
        let n: Node = Node { address: self.get_url(self.inner.lock().unwrap().communication_if.lock().unwrap().comm.own_addr.clone()), priority: self.inner.lock().unwrap().prio };
        let m: Message = Message { msg_type: msg_type as u8, payload };
        let p: Protocol = Protocol { id: guid, timestamp: self.inner.lock().unwrap().time.lock().unwrap().get_time_with_offset().to_string(), node: n, msg: m };

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

    #[allow(unused_assignments)]
    pub fn register(&mut self, e: Protocol, guid: String) -> Register {
        if e.msg.msg_type == MessageTypes::LanBroadcast as u8 {
            let msg_type: MessageTypes = MessageTypes::LanBroadcastAck;
            let payload = "".to_string();

            let n: Node = Node { address: self.get_url(self.inner.lock().unwrap().communication_if.lock().unwrap().comm.own_addr.clone()), priority: self.inner.lock().unwrap().prio };
            let m: Message = Message { msg_type: msg_type as u8, payload };
            let p: Protocol = Protocol { id: guid, timestamp: self.inner.lock().unwrap().time.lock().unwrap().get_time_with_offset().to_string(), node: n, msg: m };

            let mut r_val: Register = Register::NoneReceived;

            let f_addr_rec_d = e.node.address.clone();
            if self.inner.lock().unwrap().client_vector.contains_key(&e.node.priority) {
                print!("Client with Priority {} is already registered for address {}, dropping address {}\n", e.node.priority, self.inner.lock().unwrap().client_vector.get(&e.node.priority).unwrap().url, e.node.address);
                r_val = Register::NoneRegistered;
            } else {
                print!("Registering Client with address {}\n {} Clients connected", e.node.address, self.inner.lock().unwrap().client_vector.len());
                self.inner.lock().unwrap().client_vector.insert(e.node.priority, Client { url: e.node.address });
                r_val = Register::OneRegistered;
            }

            self.inner.lock().unwrap().communication_if.lock().unwrap().comm.foreign_addr =  f_addr_rec_d;

            match self.inner.lock().unwrap().communication_if.lock().unwrap().send_package(p) {
                Ok(_) => {}
                Err(_) => {}
            }
            return r_val;
        }
        print!("None Received!");
        return Register::NoneReceived;
    }



    pub fn run_cyclic(&mut self, guid: String) {
        let self_clone = self.inner.clone();
        loop {
            let guid_clone = guid.clone();
            match self.inner.lock().unwrap().communication_if.lock().unwrap().receive_package() {
                Ok(e) => {
                    //if self.register(e.clone(), guid_clone.clone()) as u8 != Register::NoneReceived as u8 {}
                }

                Err(_) => {}
            }
        }
    }

    pub fn run(&mut self){
        loop {
            print!("test")
        }
    }
    pub fn init(&mut self, f_addr: String, o_addr: String, timeout: u64) -> () {
        let self_clone = self.inner.clone();
        self_clone.lock().unwrap().prio = 10;
        self_clone.lock().unwrap().communication_if.lock().unwrap().comm.foreign_addr = f_addr.clone();
        self_clone.lock().unwrap().communication_if.lock().unwrap().comm.own_addr = o_addr.clone();
        self_clone.lock().unwrap().communication_if.lock().unwrap().init(timeout.clone(), true);


        let prio = self_clone.lock().unwrap().prio;
        self_clone.lock().unwrap().client_vector.insert(prio, Client { url: o_addr });


        let mut guid = helper::guid::RandomGuid { guid: "".to_string() };
        guid.create_random_guid();

        match crossbeam::scope(|scope| {
            scope.spawn(move |_| {
                self.run();
            });
        }) {
            Ok(_) => {}
            Err(_) => {}
        }

    }
}