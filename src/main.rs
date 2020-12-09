use crate::helper::json_manager::{Node, Protocol, Message};
use crate::helper::time::Time;


//mod master;
//mod slave;
mod helper;
mod master;

//use std::thread;

fn main() {
    let mut time = helper::time::Time { tim_offset: 0};
    for i in 0..1{
        time.set_auto_offset(1607557200311305);
        print!("Test {}\n", time.get_time());
        print!("Test {}\n", time.get_time_with_offset());
    }
}
