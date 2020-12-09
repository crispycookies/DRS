use std::time::Duration;
use std::net::UdpSocket;
use std::str;

pub struct Comm {
    pub(crate) socket: UdpSocket,
    pub own_addr: String,
    pub foreign_addr: String,
}

impl Comm {
    pub fn make_socket(&mut self, timeout: u64, blocking: bool) -> () {
        let timeout = Duration::from_millis(timeout);

        self.socket = UdpSocket::bind(self.own_addr.to_string()).expect("Could not connect to this address");

        let _ = self.socket.set_read_timeout(Option::from(timeout));
        let _ = self.socket.set_write_timeout(Option::from(timeout));
        let _ = self.socket.set_nonblocking(!blocking);
    }
    pub fn connect_socket(&mut self) {
        let _ = self.socket.connect(self.foreign_addr.to_string());
    }
    pub fn send(&self, data:  String) -> std::io::Result<()> {
        match self.socket.send_to(data.as_bytes(), self.foreign_addr.to_string()) {
            Err(e) => { Err(e) }
            _ => { Ok(()) }
        }
    }
    pub fn receive(&self) -> std::io::Result<String> {
        let mut buf_read = [0; 512];
        match self.socket.recv_from(&mut buf_read) {
            Err(e) => {
               Err(e)
            }
            _ => {Ok(str::from_utf8(&buf_read).unwrap().to_string())}
        }
    }

}
impl Default for Comm {
    fn default() -> Self {
        Comm {
            socket: UdpSocket::bind("127.0.0.1:34254").unwrap(),
            own_addr: "".to_string(),
            foreign_addr: "".to_string()
        }
    }
}
