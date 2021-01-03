mod comm;
mod helper;
mod master_fsm;

use crate::helper::json_manager::{Node, Message, Protocol};
//use rppal::gpio::Gpio;
//use rppal::system::{DeviceInfo};
use std::env;
use crate::master_fsm::{Master, MessageTypes, InnerMaster};
use std::time::Duration;
use std::collections::HashMap;
use rand::Rng;

fn slave() -> () {
    let args: Vec<String> = env::args().collect();
    let mut slave = comm::comm::EasyComm { comm: comm::comm_low_level::Comm::default() };
    slave.comm.foreign_addr = args.get(1).expect("expect a foreign address").to_string();
    slave.comm.own_addr = args.get(2).expect("expect own address").to_string();
    slave.init(args.get(3).expect("expect a timeout value").parse::<u64>().expect("expect a valid timeout"), true);

    let mut guid = helper::guid::RandomGuid { guid: "".to_string() };
    guid.create_random_guid();

    let time = helper::time::Time { tim_offset: std::sync::Mutex::new(0) };
    // Use the received time offset to correct the local time
    let mut rng = rand::thread_rng();
    let prio = rng.gen_range(1..100);

    let cloned_guid = guid.guid.clone();
    let n: Node = Node { address: slave.comm.own_addr.clone(), priority: prio.clone() };
    let m: Message = Message { msg_type: MessageTypes::LanBroadcast as u8, payload: "".to_string() };
    let p: Protocol = Protocol { id: cloned_guid, timestamp: time.get_time_with_offset().to_string(), node: n, msg: m };

    match slave.send_package(p.clone()) {
        Ok(_) => {
            match slave.receive_package() {
                Ok(e) => {
                    println!("Received {}", e.msg.msg_type.to_string());
                    if e.msg.msg_type == MessageTypes::LanBroadcastAck as u8 {
                        println!("Received Ack\n")
                    }
                }
                Err(_) => {}
            }
        }
        Err(_) => {
            println!("Failed to send registration package\n");
            std::thread::sleep(Duration::from_secs(1));
        }
    }

    loop{
        match slave.receive_package() {
            Ok(e) => {
                if e.msg.msg_type == MessageTypes::SendTimestamp as u8 {
                    //println!("Received Timestamp-Request from Master...\n");

                    // Store time when the message was received and send it back to master
                    // Build protocol package
                    let mut current_timestamp = time.get_time_with_offset();
                    let mut ts_response = p.clone();
                    ts_response.msg.msg_type = MessageTypes::ResponseWithTime as u8;
                    ts_response.timestamp = current_timestamp.to_string();

                    // Send timestamp back to master
                    match slave.send_package(ts_response) {
                        Ok(_) => {}
                        Err(_) => {
                            //println!("Failed to send timestamp back to Master\n");
                        }
                    }
                }
                else if e.msg.msg_type == MessageTypes::TimeOffset as u8{
                    //println!("Received TimeOffset-Request from Master...\n");

                    // Use the received time offset to correct the local time
                    let offset: i128 = e.msg.payload.parse().unwrap();
                    println!("Offset: {}", offset);
                    *time.tim_offset.lock().unwrap() = offset;
                }
            }
            Err(_) => {
                //println!("Failed to receive message...\n");
            }
        }
    }
}

fn run_pin_toggle(time: std::sync::Arc<std::sync::Mutex<helper::time::Time>>, pin: u8, desktop_mode: bool) {
    if desktop_mode {
        const SEC_DIVIDER: u128 = 6000000;
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
        //let mut gpio_pin = Gpio::new().expect("...").get(pin).expect("Wrong Pin").into_output();
        const SEC_DIVIDER: u128 = 1000000;
        const SEC_ADDER: u128 = SEC_DIVIDER / 2;
        let mut start_time = time.lock().unwrap().get_time_with_offset();
        start_time /= SEC_DIVIDER;
        start_time *= SEC_DIVIDER;
        let mut next_scheduled = start_time;
        loop {
            next_scheduled += SEC_ADDER;
            while time.lock().unwrap().get_time_with_offset() < next_scheduled {}
            /*if gpio_pin.is_set_high() {
                gpio_pin.set_low();
            } else {
                gpio_pin.set_high();
            }*/
            println!("Test {}:{}:s-{}", time.lock().unwrap().get_time_with_offset(), next_scheduled, start_time);
        }
    }
}

fn main() {
    let time_arc = std::sync::Arc::new(std::sync::Mutex::new(helper::time::Time { tim_offset: std::sync::Mutex::new(0) }));
    let time_arc_for_thread = time_arc.clone();
    let time_arc_for_master_run_thread = time_arc.clone();
    let args: Vec<String> = env::args().collect();

    /*match DeviceInfo::new() {
        Ok(e) => {
            println!("Running DRS on {}.", e.model());
        }
        Err(_) => {
            println!("Running on Generic Desktop PC or similar");
        }
    }*/

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
                client_vector: HashMap::new()
            }
        };
    //args.get(1).expect("expect a foreign address").to_string()
        master.init("".to_string(),
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
