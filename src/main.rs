use piper;
use std::convert::From;
use std::env;
use std::fmt::Debug;
use std::io::Write;
use std::time::Duration;

use linefeed::{Interface, ReadResult, Terminal};

use serde_json;

//use serialport::open;
//use serialport::{DataBits, FlowControl, Parity, SerialPort, SerialPortSettings, StopBits};
//
use postcard::{self, from_bytes_cobs, to_slice_cobs};
use serde::{Deserialize, Serialize};

use mio_serial;
use mio_serial::{DataBits, FlowControl, Parity, Serial, SerialPort, SerialPortSettings, StopBits};

use smol::{self, blocking, Async, Timer};

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

fn main() -> Result<(), Error> {
    let path = env::args()
        .skip(1)
        .next()
        .expect("Expected path to serial device");
    // Open the first serialport available.
    let mut portw_ = Serial::from_path(&path, &SETTINGS).expect("Failed to open serial port");

    let mut portw = piper::Arc::new(Async::new(portw_).unwrap());
    let mut portr = portw.clone();

    let (mut tx, rx) = piper::chan::<Message>(100);

    let interface = piper::Arc::new(Interface::new("async-demo")?);
    interface.set_prompt("async-demo> ")?;

    let iface = interface.clone();

    smol::Task::spawn(async { async_write_usart(iface, portw, rx).await })
        .unwrap()
        .detach();

    let iface = interface.clone();
    smol::Task::spawn(async { async_read_usart(iface, portr).await })
        .unwrap()
        .detach();

    smol::Task::blocking(async { cli(interface, tx).await })
        .unwrap()
        .detach();

    smol::run(future::pending::<()>());
    Ok(())
}

async fn cli<T: Terminal>(
    interface: piper::Arc<Interface<T>>,
    tx: piper::Sender<Message>,
) -> Result<(), Error> {
    while let ReadResult::Input(input) = interface.read_line()? {
        writeln!(interface, "got input {:?}", input);
        interface.add_history_unique(input.clone());

        match serde_json::from_str(&input) {
            Ok(m) => {
                tx.send(m).await;
            }
            Err(e) => {
                writeln!(interface, "cannot parse command {:?}", e);
            }
        }
    }
    Ok(())
}

async fn async_write_usart<T: Terminal>(
    term: piper::Arc<Interface<T>>,
    mut port: impl AsyncWrite + Unpin,
    rx: piper::Receiver<Message>,
) -> Result<(), Error> {
    writeln!(term, "Start writer");
    let mut buf: [u8; 32] = [0; 32];

    while let Some(msg) = rx.recv().await {
        writeln!(term, "try to send msg = {:?}", msg);
        let used = to_slice_cobs(&msg, &mut buf[..])?;
        port.write_all(&used).await?;
    }
    Ok(())
}

async fn async_read_usart<T: Terminal>(
    term: piper::Arc<Interface<T>>,
    port: impl AsyncRead + Unpin,
) -> Result<(), Error> {
    writeln!(term, "start reader");
    //let mut pool = Pool::with_capacity(5, 0, || Vec::with_capacity(512));
    let mut reader = BufReader::new(port);
    let mut n = 0;
    let mut vec: Vec<u8> = Vec::with_capacity(512);

    loop {
        vec.clear();
        n = reader.read_until(0u8, &mut vec).await?;
        let msg: Message = from_bytes_cobs(vec.as_mut_slice())?;
        writeln!(term, "msg = {:?}", msg);
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
