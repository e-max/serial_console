use serialport;

use piper;
use std::env;
use std::io;
use std::io::{Read, Write};
use std::time::Duration;
use std::{mem, thread};

//use serialport::open;
//use serialport::{DataBits, FlowControl, Parity, SerialPort, SerialPortSettings, StopBits};

use mio_serial;
use mio_serial::{DataBits, FlowControl, Parity, Serial, SerialPort, SerialPortSettings, StopBits};

use smol::{self, Async, Timer};

use futures::future;
use futures::io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader};

//const SETTINGS: SerialPortSettings = SerialPortSettings {
//baud_rate: 115200,
//data_bits: DataBits::Eight,
//parity: Parity::None,
//stop_bits: StopBits::One,
//flow_control: FlowControl::None,
//timeout: Duration::from_secs(3600),
//};

const SETTINGS: SerialPortSettings = mio_serial::SerialPortSettings {
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
    let mut portw_ = Serial::from_path(&path, &SETTINGS).expect("Failed to open serial port");

    let mut portw = piper::Arc::new(Async::new(portw_).unwrap());
    let mut portr = portw.clone();

    smol::Task::spawn(async { async_write_usart(portw).await })
        .unwrap()
        .detach();

    smol::Task::spawn(async { async_read_usart(portr).await })
        .unwrap()
        .detach();
    smol::run(future::pending::<()>());

    // Send out 4 bytes every second
    //thread::spawn(move || loop {
    //clone
    //.write(&[5, 6, 7, 8])
    //.expect("Failed to write to serial port");
    //thread::sleep(Duration::from_millis(1000));
    //});

    // Read the four bytes back from the cloned port
    //
    //read_usart(portr);
    println!("Hello, world!");
}
async fn async_write_usart(mut port: impl AsyncWrite + Unpin) -> Result<(), std::io::Error> {
    println!("Start writer");
    loop {
        port.write_all("Hi\0".as_bytes()).await?;
        println!("wrote hi");
        Timer::after(Duration::from_secs(1)).await;
    }
}

async fn async_read_usart(port: impl AsyncRead + Unpin) -> Result<(), std::io::Error> {
    println!("start reader");
    //let mut pool = Pool::with_capacity(5, 0, || Vec::with_capacity(512));
    let mut reader = BufReader::new(port);
    let mut n = 0;
    let mut vec: Vec<u8> = Vec::with_capacity(512);

    loop {
        vec.clear();
        n = reader.read_until(0u8, &mut vec).await?;
        vec.truncate(n - 1);
        let s = String::from_utf8(vec.clone());
        println!("s = {:?}", s);
    }
}

//fn read_usart(mut port: Box<SerialPort>) {
////let mut pool = Pool::with_capacity(5, 0, || Vec::with_capacity(512));
//let mut reader = BufReader::new(port);
//let mut n = 0;
//let mut vec: Vec<u8> = Vec::with_capacity(512);

//loop {
//vec.clear();
//n = reader.read_until(0u8, &mut vec).unwrap();
//vec.truncate(n - 1);
//let s = String::from_utf8(vec.clone());
//println!("s = {:?}", s);
//}
//}
