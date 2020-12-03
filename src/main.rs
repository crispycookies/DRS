//mod master;
//mod slave;
mod helper;

//use std::thread;

fn main() {
    println!("{}", helper::json_manager::get_guid());
    println!("{}", helper::json_manager::get_timestamp());
    println!("{}", helper::json_manager::serialize());
    helper::json_manager::deserialize("hallo");
}
