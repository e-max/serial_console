use serialport;

use std::env;
use std::io;
use std::io::{BufRead, BufReader};
use std::io::{Read, Write};
use std::time::Duration;
use std::{mem, thread};

use serialport::open;
use serialport::{DataBits, FlowControl, Parity, SerialPort, SerialPortSettings, StopBits};

const SETTINGS: SerialPortSettings = SerialPortSettings {
    baud_rate: 115200,
    data_bits: DataBits::Eight,
    parity: Parity::None,
    stop_bits: StopBits::One,
    flow_control: FlowControl::None,
    timeout: Duration::from_secs(3600),
};

fn main() {
    let path = env::args()
        .skip(1)
        .next()
        .expect("Expected path to serial device");
    // Open the first serialport available.
    let mut portw =
        serialport::open_with_settings(&path, &SETTINGS).expect("Failed to open serial port");
    let mut portr = portw.try_clone().unwrap();

    // Send out 4 bytes every second
    //thread::spawn(move || loop {
    //clone
    //.write(&[5, 6, 7, 8])
    //.expect("Failed to write to serial port");
    //thread::sleep(Duration::from_millis(1000));
    //});

    // Read the four bytes back from the cloned port
    //
    read_usart(portr);
    println!("Hello, world!");
}

fn read_usart(mut port: Box<SerialPort>) {
    //let mut pool = Pool::with_capacity(5, 0, || Vec::with_capacity(512));
    let mut reader = BufReader::new(port);
    let mut n = 0;
    let mut vec: Vec<u8> = Vec::with_capacity(512);

    loop {
        vec.clear();
        n = reader.read_until(0u8, &mut vec).unwrap();
        vec.truncate(n - 1);
        let s = String::from_utf8(vec.clone());
        println!("s = {:?}", s);
    }
}
