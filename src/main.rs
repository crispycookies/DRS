mod comm;
mod helper;

use crate::helper::json_manager::{Node, Message, Protocol};
use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;
use std::env;
use std::time::Duration;

fn master() -> () {
    let args: Vec<String> = env::args().collect();
    let mut master = comm::comm::EasyComm { comm: comm::comm_low_level::Comm::default() };
    master.comm.foreign_addr = args.get(1).expect("expect a foreign address").to_string();
    master.comm.own_addr = args.get(2).expect("expect own address").to_string();
    master.init(args.get(3).expect("expect a timeout value").parse::<u64>().expect("expect a valid timeout"), true);

    let guid = helper::guid::RandomGuid{ guid : "".to_string()};

    for _ in 0..1000 {
        let string  = guid.guid;
        let n: Node = Node { address: "Hallo".to_string(), priority: 0x3 };
        let m: Message = Message { msg_type: 0x1, payload: "Seas".to_string() };
        let p: Protocol = Protocol { id: guid.guid, timestamp: "Seas".to_string(), node: n, msg: m };

        match master.send_package(p) {
            Ok(_) => { print!("Succeeded in Sending\n") }
            Err(_) => { print!("Failed to send\n") }
        }
    }
}

fn slave() -> () {
    const GPIO_LED: u8 = 12;
    let mut pin = Gpio::new().expect("...").get(GPIO_LED).expect("...").into_output();


    let args: Vec<String> = env::args().collect();
    let mut master = comm::comm::EasyComm { comm: comm::comm_low_level::Comm::default() };
    master.comm.foreign_addr = args.get(1).expect("expect a foreign address").to_string();
    master.comm.own_addr = args.get(2).expect("expect own address").to_string();
    master.init(args.get(3).expect("expect a timeout value").parse::<u64>().expect("expect a valid timeout"), true);


    for _ in 0..1000 {
        match master.receive_package() {
            Ok(e) => {
                print!("received package {}", helper::json_manager::serialize(e))
            }
            Err(e) => {
                print!("No Package received or package damaged!\n");
                print!("{}", e);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Running DRS on {}.", DeviceInfo::new().unwrap().model());


    if args.get(4).unwrap() == "master" {
        master();
    } else if args.get(4).unwrap() == "slave" {
        slave();
    } else {
        panic!("DRS must be either Slave or Master");
    }
}
