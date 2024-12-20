use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;
use crate::println;

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

pub fn init() {
    println!("Serial driver initialized");
}
