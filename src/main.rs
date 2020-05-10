use serialport;

use piper;
use std::convert::From;
use std::env;
use std::fmt::Debug;
use std::io;
use std::io::{Read, Write};
use std::time::Duration;
use std::{mem, thread};

//use serialport::open;
//use serialport::{DataBits, FlowControl, Parity, SerialPort, SerialPortSettings, StopBits};
//
use postcard::{self, from_bytes_cobs, to_slice_cobs};
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Debug)]
enum Message {
    Msg(String),
    Coord { x: u32, y: u32 },
    List(Vec<u8>),
}

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
async fn async_write_usart(mut port: impl AsyncWrite + Unpin) -> Result<(), Error> {
    println!("Start writer");
    let mut buf: [u8; 32] = [0; 32];
    let my_msg = Message::Msg("hello".to_owned());
    let my_coord = Message::Coord { x: 32, y: 12 };
    let my_list = Message::List(vec![1, 2, 3]);

    loop {
        let used = to_slice_cobs(&my_msg, &mut buf[..])?;
        port.write_all(&used).await?;
        let used = to_slice_cobs(&my_coord, &mut buf[..])?;
        port.write_all(&used).await?;
        let used = to_slice_cobs(&my_list, &mut buf[..])?;
        port.write_all(&used).await?;
        Timer::after(Duration::from_secs(1)).await;
    }
}

async fn async_read_usart(port: impl AsyncRead + Unpin) -> Result<(), Error> {
    println!("start reader");
    //let mut pool = Pool::with_capacity(5, 0, || Vec::with_capacity(512));
    let mut reader = BufReader::new(port);
    let mut n = 0;
    let mut vec: Vec<u8> = Vec::with_capacity(512);

    loop {
        vec.clear();
        n = reader.read_until(0u8, &mut vec).await?;
        let msg: Message = from_bytes_cobs(vec.as_mut_slice())?;
        println!("msg = {:?}", msg);
    }
}

#[derive(Debug)]
struct Error(String);

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error(format!("{:?}", e))
    }
}

impl From<postcard::Error> for Error {
    fn from(e: postcard::Error) -> Self {
        Error(format!("{:?}", e))
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
