use crate::comm::comm_low_level::Comm;
use crate::helper::json_manager::{Protocol, serialize, deserialize};
use std::io::Error;


struct EasyComm {
    pub comm : Comm
}
impl EasyComm {
    pub fn send_package(&self, protocol : Protocol) -> std::io::Result<bool>{
        let serialized = serialize(protocol);
        let mut recvd = "".to_string();
        match  self.comm.send(serialized) {
            Ok(_) => {Ok(true)}
            Err(e) => {Err(e)}
        }
        match self.comm.receive() {
            Ok(e) => {recvd = e}
            Err(_) => {Ok(false)}
        }
        match deserialize(recvd) {
            Ok(e) => {
                if(e.msg.msg_type == 9){
                    Ok(true)
                }
                Ok(false)
            }
            Err(e) => {Err(e)}
        }
        Ok(false)
    }
}