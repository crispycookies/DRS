use crate::comm::comm_low_level::Comm;
use crate::helper::json_manager::{Protocol, serialize, deserialize};

pub struct EasyComm {
    pub comm: Comm
}

impl EasyComm {
    #[allow(dead_code)]
    pub fn init(&mut self, timeout : u64, blocking : bool) -> (){
        self.comm.make_socket(timeout, blocking);
        self.comm.connect_socket();
    }
    #[allow(dead_code)]
    pub fn send_package(&self, protocol: Protocol) -> std::io::Result<()> {
        let serialized = serialize(protocol);
        print!("{}",serialized);
        match self.comm.send(serialized) {
            Ok(_) => { Ok(()) }
            Err(e) => { Err(e) }
        }
    }
    #[allow(dead_code)]
    pub fn receive_package(&self) -> std::io::Result<Protocol> {
        match self.comm.receive() {
            Ok(e) => {
                match deserialize(e){
                    Ok(e) => {Ok(e)}
                    Err(e) => {Err(e)}
                }
            }
            Err(e) => { Err(e) }
        }
    }
}