mod master;
mod slave;
mod time;

use std::thread;

fn main() {
    let child = thread::spawn(move || {
       let _ = master::test();
    });
    let child2 = thread::spawn(move || {
        let _ = slave::test2();
    });
// some work here
    let _ = child.join();
// some work here
    let _ = child2.join();
}
