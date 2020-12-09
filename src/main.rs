mod comm;

use std::{thread, time};

fn test1() -> () {
    let mut test = comm::comm_low_level::Comm::default();
    test.foreign_addr = "127.0.0.1:1884".to_string();
    test.own_addr = "127.0.0.1:1882".to_string();
    test.make_socket(100, true);
    test.connect_socket();

    for _ in 0..1000 {
       match test.send(&"Hallo".to_string()){
           Ok(_) => {print!("Succeeded in Sending\n")}
           Err(_) => {print!("Failed to send\n")}
       }
    }
}

fn test2() -> () {
    let mut test = comm::comm_low_level::Comm::default();
    test.foreign_addr = "127.0.0.1:1880".to_string();
    test.own_addr = "127.0.0.1:1881".to_string();
    test.make_socket(100, true);
    test.connect_socket();

    for _ in 0..1000 {
        match test.receive(){
            Ok(e) => {print!("Received: {}", e)}
            Err(_) => {print!("Received nothing\n")}
        }
    }
}

fn main() {
    let ten_millis = time::Duration::from_millis(10);
    let t1 = thread::spawn(move || {
        test1();
    });
    let t2 = thread::spawn(move || {
        test2();
    });
    let _ = t1.join();
    let _ = t2.join();
}
