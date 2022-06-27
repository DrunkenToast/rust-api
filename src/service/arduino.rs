pub mod actions;
use std::thread;
use std::time::Duration;
use std::{io::Write, sync::Arc};
use std::io::{self, Read};
use tokio_serial::{SerialPort, Parity};
use tokio::sync::Mutex;
use tokio::sync::broadcast::{self, Sender, Receiver};

use crate::model::error::ArduinoError;

#[repr(u8)]
enum Action {
    Hello = 0,
    SwitchLed = 1,
    DisplayMessage = 2,
    ReadDHT = 3,
    Recv = 4,
}

impl TryFrom<u8> for Action {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Action::Hello),
            1 => Ok(Action::SwitchLed),
            2 => Ok(Action::DisplayMessage),
            3 => Ok(Action::ReadDHT),
            4 => Ok(Action::Recv),
            _ => Err(()),
        }
    }
}

pub type ArduinoState = Arc<Mutex<Arduino<>>>;

pub struct Arduino {
    serial: Box<dyn SerialPort>,
    rx: Receiver<u8>,
}

impl Arduino {
    pub async fn new(port: String) -> Self {
        let (tx, rx): (Sender<u8>, Receiver<u8>) = broadcast::channel(16);
        let mut rx2 = tx.subscribe();
        let serial = tokio_serial::new(&port, 9600)
            .parity(Parity::None).timeout(Duration::from_millis(100))
            .open().expect("Failed to open port");
        let mut serial2 = serial.try_clone().expect("clone serial failed");

        let mut con = Self {
            serial,
            rx,
        };

        thread::spawn(move || {
            loop {
                let mut buf = [0; 1];
                if serial2.read(&mut buf).is_ok() {
                    tx.send(buf[0]).unwrap();
                }
            }
        });

        // Watch incoming serial stream 
        tokio::spawn(async move {
            loop {
                if let Ok(val) = rx2.recv().await {
                    println!("=> {:?}", val);
                }
            }
        });

        println!("Waiting for hello signal");
        loop {
            if let Ok(val) = con.rx.recv().await{
                match Action::try_from(val) {
                    Ok(Action::Hello) => {
                        con.write_action(Action::Hello).unwrap();
                    },
                    Ok(Action::Recv) => {
                        con.rx = con.rx.resubscribe();
                        break;
                    },
                    _ => {}
                }
            }
        }
        println!("Connection has been made with arduino.");
        con
    }

    fn write_action(&mut self, action: Action) -> Result<(), io::Error> {
        self.write_u8(action as u8)?;
        Ok(())
    }

    fn write_u8(&mut self, b: u8) -> Result<(), io::Error> {
        let buf = [b];
        self.serial.write_all(&buf)?;
        self.serial.flush()?;
        Ok(())
    }

    async fn read_or_timeout(&mut self) -> Result<u8, ArduinoError> {
        //if let Ok(Ok(val)) = timeout(Duration::from_millis(5000), self.rx.recv()).await {
            //Ok(val)
        //}    
        //else {Err(ArduinoError::Timeout)}
        match self.rx.recv().await {
            Ok(val) => {
                Ok(val)
            }
            Err(e) => {
                println!("AAAAAAAAAAAAAAAAA {:?}", e);
                Err(ArduinoError::IoError)
            }
        }
        //Ok(self.rx.recv().await.unwrap())
    }
}