use std::net::{UdpSocket};
use std::env;
use std::time::{Duration};

pub fn test2() -> std::io::Result<()> {
    {
        let args: Vec<String> = env::args().collect();
        if args.len() != 3{
            panic!("Invalid Count of Arguments Provided")
        }

        print!("Connecting\n");
        let timeout = Duration::from_millis(10000);
        let socket = UdpSocket::bind(args.get(1).unwrap().to_string()).expect("Could not connect to Device, is it correct?");
        let _timout_socket = socket.set_read_timeout(Option::from(timeout));
        let _timout_socket = socket.set_write_timeout(Option::from(timeout));

        let _block = socket.set_nonblocking(false);

        let _c = socket.connect(args.get(2).unwrap().to_string());


        print!("Running\n");

        loop {
            // Receives a single datagram message on the socket. If `buf` is too small to hold
            // the message, it will be cut off.
            let _buf_send = [255; 16];
            let mut buf_read = [0; 16];

            match socket.recv_from(&mut buf_read){
                Err(_e) => {
                }
                _ => { match socket.send_to(&buf_read, args.get(2).unwrap().to_string()){
                    Err(_e) => {

                    }
                    _ => {
                        //print!("Sent\n")
                    }
                }}
            }
        }
    } // the socket is closed here
    Ok(())
}
