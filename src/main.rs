mod comm;
mod helper;

use crate::helper::json_manager::{Node, Message, Protocol};
//use rppal::gpio::Gpio;
//use rppal::system::DeviceInfo;
use std::env;
use std::time::Duration;
use std::borrow::Borrow;

enum MessageTypes{
    LanBroadcast = 0x1,
    LanBroadcastAck = 0x2,
    BroadcastNewMaster = 0x3,
    BroadcastNewMasterAck = 0x4,
    SendTimestamp = 0x5,
    ResponseWithTime = 0x6,
    TimeOffset = 0x7,
    Invalid = 0x99
}



fn master() -> () {
    let args: Vec<String> = env::args().collect();
    let mut master = comm::comm::EasyComm { comm: comm::comm_low_level::Comm::default() };
    master.comm.foreign_addr = args.get(1).expect("expect a foreign address").to_string();
    master.comm.own_addr = args.get(2).expect("expect own address").to_string();
    master.init(args.get(3).expect("expect a timeout value").parse::<u64>().expect("expect a valid timeout"), true);

    let mut guid = helper::guid::RandomGuid{ guid : "".to_string()};
    guid.create_random_guid();

    let time = helper::time::Time {tim_offset: 0};
    let prio = 0x3;

    loop {
        let cloned_guid  = guid.guid.clone();
        let n: Node = Node { address: args.get(2).unwrap().to_string(), priority: 0x3 };
        let m: Message = Message { msg_type: 0x3, payload: "hallo".to_string() };
        let p: Protocol = Protocol { id: cloned_guid, timestamp: time.get_time_with_offset().to_string(), node: n, msg: m };

        match master.send_package(p) {
            Ok(_) => { print!("Succeeded in Sending\n") }
            Err(_) => { print!("Failed to send\n") }
        }
        /*
        match master.receive_package() {
            Ok(e) => {
                print!("received package {}", helper::json_manager::serialize(e.clone()));
                if e.msg.msg_type == 0x6 {
                    let cloned_guid  = guid.guid.clone();
                    let n: Node = Node { address: args.get(2).unwrap().to_string(), priority: prio.clone() };
                    let m: Message = Message { msg_type: 0x7, payload: "".to_string() };
                    let p: Protocol = Protocol { id: cloned_guid, timestamp: time.get_time_with_offset().to_string(), node: n, msg: m };

                    match master.send_package(p) {
                        Ok(_) => { print!("Succeeded in Sending\n") }
                        Err(_) => { print!("Failed to send\n") }
                    }
                }
            }
            Err(e) => {
                print!("No Package received or package damaged!\n");
                print!("{}", e);
            }
        }
        */
        //std::thread::sleep(Duration::from_millis(1000));
    }
}

fn slave() -> () {
    //const GPIO_LED: u8 = 12;
    //let mut pin = Gpio::new().expect("...").get(GPIO_LED).expect("...").into_output();


    let args: Vec<String> = env::args().collect();
    let mut slave = comm::comm::EasyComm { comm: comm::comm_low_level::Comm::default() };
    slave.comm.foreign_addr = args.get(1).expect("expect a foreign address").to_string();
    slave.comm.own_addr = args.get(2).expect("expect own address").to_string();
    slave.init(args.get(3).expect("expect a timeout value").parse::<u64>().expect("expect a valid timeout"), true);

    let mut guid = helper::guid::RandomGuid{ guid : "".to_string()};
    guid.create_random_guid();

    let mut time = helper::time::Time {tim_offset: 0};
    let prio = 0x3;


    loop {
        match slave.receive_package() {
            Ok(e) => {

                print!("received package {}", helper::json_manager::serialize(e.clone()));
                if e.msg.msg_type == 0x5 {
                    let cloned_guid  = guid.guid.clone();
                    let n: Node = Node { address: args.get(1).unwrap().to_string(), priority: prio.clone() };
                    let m: Message = Message { msg_type: 0x6, payload: "".to_string() };
                    let p: Protocol = Protocol { id: cloned_guid, timestamp: time.get_time_with_offset().to_string(), node: n, msg: m };

                    match slave.send_package(p) {
                        Ok(_) => { print!("Succeeded in Sending\n") }
                        Err(_) => { print!("Failed to send\n") }
                    }
                }

            }
            Err(e) => {
                print!("No Package received or package damaged!\n");
                print!("{}", e);
            }
        }
/*
        match slave.receive_package() {
            Ok(e) => {
                print!("received package {}", helper::json_manager::serialize(e.clone()));
                if e.msg.msg_type == 0x7 {
                }
            }
            Err(e) => {
                print!("No Package received or package damaged!\n");
                print!("{}", e);
            }
        }

 */
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    //println!("Running DRS on {}.", DeviceInfo::new().unwrap().model());


    if args.get(4).unwrap() == "master" {
        master();
    } else if args.get(4).unwrap() == "slave" {
        slave();
    } else {
        panic!("DRS must be either Slave or Master");
    }
}
