use crate::comm::comm_low_level::Comm;
use crate::helper::json_manager::{Protocol, serialize, deserialize, Node, Message};
use std::io::Error;


struct EasyComm {
    pub comm: Comm
}

impl EasyComm {
    pub fn send_package(&self, protocol: Protocol) -> std::io::Result<()> {
        let serialized = serialize(protocol);
        match self.comm.send(serialized) {
            Ok(_) => { Ok(()) }
            Err(e) => { Err(e) }
        }
    }
    pub fn receive_package(&self) -> std::io::Result<Protocol> {
        let mut recvd = "".to_string();
        match self.comm.receive() {
            Ok(e) => {
                recvd = e;
                match deserialize(recvd){
                    Ok(e) => {Ok((e))}
                    Err(e) => {Err(e)}
                }
            }
            Err(e) => { Err(e) }
        }
    }
}