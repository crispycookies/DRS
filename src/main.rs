mod comm;
mod helper;
mod master_fsm;

use crate::helper::json_manager::{Node, Message, Protocol};
use rppal::gpio::Gpio;
use rppal::system::{DeviceInfo};
use std::env;
use crate::master_fsm::{Master, MessageTypes, InnerMaster};
use std::time::Duration;
use std::collections::HashMap;

fn slave() -> () {
    let args: Vec<String> = env::args().collect();
    let mut slave = comm::comm::EasyComm { comm: comm::comm_low_level::Comm::default() };
    slave.comm.foreign_addr = args.get(1).expect("expect a foreign address").to_string();
    slave.comm.own_addr = args.get(2).expect("expect own address").to_string();
    slave.init(args.get(3).expect("expect a timeout value").parse::<u64>().expect("expect a valid timeout"), true);

    let mut guid = helper::guid::RandomGuid { guid: "".to_string() };
    guid.create_random_guid();

    let time = helper::time::Time { tim_offset: std::sync::Mutex::new(0) };
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

fn run_pin_toggle(time: std::sync::Arc<std::sync::Mutex<helper::time::Time>>, pin: u8, desktop_mode: bool) {
    if desktop_mode {
        const SEC_DIVIDER: u128 = 1000000;
        const SEC_ADDER: u128 = SEC_DIVIDER / 2;

        let mut start_time = time.lock().unwrap().get_time_with_offset();
        start_time /= SEC_DIVIDER;
        start_time *= SEC_DIVIDER;
        let mut next_scheduled = start_time;
        loop {
            next_scheduled += SEC_ADDER;
            while time.lock().unwrap().get_time_with_offset() < next_scheduled {}
            println!("Test {}:{}:s-{}", time.lock().unwrap().get_time_with_offset(), next_scheduled, start_time);
        }
    } else {
        let mut gpio_pin = Gpio::new().expect("...").get(pin).expect("Wrong Pin").into_output();
        const SEC_DIVIDER: u128 = 1000000;
        const SEC_ADDER: u128 = SEC_DIVIDER / 2;
        let mut start_time = time.lock().unwrap().get_time_with_offset();
        start_time /= SEC_DIVIDER;
        start_time *= SEC_DIVIDER;
        let mut next_scheduled = start_time;
        loop {
            next_scheduled += SEC_ADDER;
            while time.lock().unwrap().get_time_with_offset() < next_scheduled {}
            if gpio_pin.is_set_high() {
                gpio_pin.set_low();
            } else {
                gpio_pin.set_high();
            }
            println!("Test {}:{}:s-{}", time.lock().unwrap().get_time_with_offset(), next_scheduled, start_time);
        }
    }
}

fn main() {
    let time_arc = std::sync::Arc::new(std::sync::Mutex::new(helper::time::Time { tim_offset: std::sync::Mutex::new(0) }));
    let time_arc_for_thread = time_arc.clone();
    let time_arc_for_master_run_thread = time_arc.clone();
    let args: Vec<String> = env::args().collect();

    match DeviceInfo::new() {
        Ok(e) => {
            println!("Running DRS on {}.", e.model());
        }
        Err(_) => {
            println!("Running on Generic Desktop PC or similar");
        }
    }

    let spawn = std::thread::spawn(move || {
        run_pin_toggle(time_arc_for_thread, 12, true);
    });

    if args.get(4).unwrap() == "master" {
        let cf = comm::comm::EasyComm {comm : comm::comm_low_level::Comm::default()};
        let mut master = Master {
            inner: InnerMaster {
                communication_if: cf,
                time: time_arc_for_master_run_thread,
                prio: 0xFF,
                client_vector: std::sync::Mutex::new(HashMap::new())
            }
        };

        master.init(args.get(1).expect("expect a foreign address").to_string(),
                    args.get(2).expect("expect own address").to_string(),
                    args.get(3).expect("expect a timeout value").
                        parse::<u64>().expect("expect a valid timeout"));

    } else if args.get(4).unwrap() == "slave" {
        slave();
    } else {
        panic!("DRS must be either Slave or Master");
    }

    match spawn.join() {
        Ok(_) => {}
        Err(_) => {}
    }
}
