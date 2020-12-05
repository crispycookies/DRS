use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

const GPIO_LED: u8 = 12;

fn main() {
    println!("Blinking an LED on a {}.", DeviceInfo::new().expect("Could not fetch device").model());

    let mut pin = Gpio::new().expect("Test").get(GPIO_LED).expect("Test").into_output();

    // Blink the LED by setting the pin's logic level high for 500 ms.
    while true {
        pin.set_high();
        thread::sleep(Duration::from_millis(500));
        pin.set_low();
        thread::sleep(Duration::from_millis(500));
    }

}
