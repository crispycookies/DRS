mod comm;
mod helper;
mod master_fsm;

use crate::helper::json_manager::{Node, Message, Protocol};
use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;
use std::env;
use crate::master_fsm::{Master, MessageTypes};
use std::time::Duration;
use std::collections::HashMap;

fn slave() -> () {
    //const GPIO_LED: u8 = 12;
    //let mut pin = Gpio::new().expect("...").get(GPIO_LED).expect("...").into_output();


    let args: Vec<String> = env::args().collect();
    let mut slave = comm::comm::EasyComm { comm: comm::comm_low_level::Comm::default() };
    slave.comm.foreign_addr = args.get(1).expect("expect a foreign address").to_string();
    slave.comm.own_addr = args.get(2).expect("expect own address").to_string();
    slave.init(args.get(3).expect("expect a timeout value").parse::<u64>().expect("expect a valid timeout"), true);

    let mut guid = helper::guid::RandomGuid { guid: "".to_string() };
    guid.create_random_guid();

    let time = helper::time::Time { tim_offset: 0 };
    let prio = 0x3;


    let cloned_guid = guid.guid.clone();
    let n: Node = Node { address: args.get(1).unwrap().to_string(), priority: prio.clone() };
    let m: Message = Message { msg_type: MessageTypes::LanBroadcast as u8, payload: "".to_string() };
    let p: Protocol = Protocol { id: cloned_guid, timestamp: time.get_time_with_offset().to_string(), node: n, msg: m };

    loop {
        match slave.send_package(p.clone()) {
            Ok(_) => {
                match slave.receive_package() {
                    Ok(e) => {
                        if e.msg.msg_type == MessageTypes::LanBroadcastAck as u8 {
                            print!("Received Ack\n")
                        }
                    }
                    Err(_) => {}
                }
            }
            Err(_) => {
                print!("Failed to send package\n");
                std::thread::sleep(Duration::from_secs(1));
            }
        }


       /* match slave.receive_package() {
            Ok(e) => {
                print!("received package {}", helper::json_manager::serialize(e.clone()));
                if e.msg.msg_type == 0x03 {
                    let cloned_guid = guid.guid.clone();
                    let n: Node = Node { address: args.get(1).unwrap().to_string(), priority: prio.clone() };
                    let m: Message = Message { msg_type: 0x04, payload: "".to_string() };
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
        */
    }
}

fn main() {
    //println!("Running DRS on {}.", DeviceInfo::new().unwrap().model());

    let args: Vec<String> = env::args().collect();
    if args.get(4).unwrap() == "master" {
        let mut master = Master { communication_if: comm::comm::EasyComm {
            comm: comm::comm_low_level::Comm::default() },
            time: helper::time::Time { tim_offset: 0 },
            prio: 0x3, client_vector : HashMap::new()
        };

        master.run(args.get(1).expect("expect a foreign address").to_string(),
                   args.get(2).expect("expect own address").to_string(),
                   args.get(3).expect("expect a timeout value").
                       parse::<u64>().expect("expect a valid timeout"), 255)


    } else if args.get(4).unwrap() == "slave" {
        slave();
    } else {
        panic!("DRS must be either Slave or Master");
    }
}
