//mod master;
//mod slave;
mod helper;

//use std::thread;

fn main() {
    print!("{}", helper::json_manager::get_timestamp());
}
