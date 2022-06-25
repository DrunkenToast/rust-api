use std::future::Future;
use std::thread;
use std::time::Duration;
use std::{io::Write, sync::Arc};
use std::io::{self, Read};
use tokio::time::timeout;
use tokio_serial::{SerialPort, Parity};
use tokio::sync::Mutex;
use tokio::sync::broadcast::{self, Sender, Receiver};

use crate::model::health::HealthStatus;

pub enum ArduinoError {
    Timeout,
    IoError,
    RecvError,
}

#[repr(u8)]
enum Action {
    Hello = 0,
    SwitchLed = 1,
    DisplayMessage = 2,
    ReadTemperature = 3,
    ReadHumidity = 4,
    Recv = 5,
}

impl TryFrom<u8> for Action {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Action::Hello),
            1 => Ok(Action::SwitchLed),
            2 => Ok(Action::DisplayMessage),
            3 => Ok(Action::ReadTemperature),
            4 => Ok(Action::ReadHumidity),
            5 => Ok(Action::Recv),
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
        let (tx, mut rx): (Sender<u8>, Receiver<u8>) = broadcast::channel(16);
        let rx2 = tx.subscribe();
        let serial = tokio_serial::new(&port, 115200)
            .parity(Parity::None).timeout(Duration::from_millis(100))
            .open().expect("Failed to open port");
        let mut serial2 = serial.try_clone().expect("clone serial failed");

        let mut con = Self {
            serial,
            rx: rx2
        };

        thread::spawn(move || {
            loop {
                let mut buf = [0 as u8; 1];
                if serial2.read(&mut buf).is_ok() {
                    tx.send(buf[0]).unwrap();
                }
            }
        });

        // Watch incoming serial stream 
        tokio::spawn(async move {
            loop {
                if let Ok(val) = rx.recv().await {
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

    async fn read_done_or_timeout(&mut self) -> Result<(), ArduinoError> {
        if let Ok(Ok(val)) = timeout(Duration::from_millis(500), self.rx.recv()).await {
            if val == Action::Recv as u8 {
                Ok(())
            }
            else {Err(ArduinoError::RecvError)}
        }
        else {Err(ArduinoError::Timeout)}
    }

    pub async fn switch_led(&mut self, state: bool) -> Result<(), ArduinoError> {
        if self.write_action(Action::SwitchLed).is_ok() {
            println!("Switching led to {:?}", state as u8);
            self.write_u8(state as u8).ok();
            self.read_done_or_timeout().await
        }
        else {Err(ArduinoError::IoError)}
    }

    pub async fn health(&mut self) -> HealthStatus {
        if self.write_action(Action::Hello).is_ok() 
        && self.read_done_or_timeout().await.is_ok() {
            return HealthStatus::Up
        }
        HealthStatus::Down
    }

    // Displays a message on the LCD screen of the arduino
    // Will first write a display message action, then the amount of bytes (clamped to 32 bytes).
    // After that the message itself is written.
    pub async fn display_message(&mut self, message: &str) -> Result<(), ArduinoError> {
        let message = message.as_bytes();
        let message = &message[..31.clamp(0, message.len())];
        if let Ok(_) = self.write_action(Action::DisplayMessage) {
            self.serial.write(&[message.len() as u8]).ok();
            self.serial.write(message).ok();
            self.read_done_or_timeout().await
        }
        else {Err(ArduinoError::IoError)}
    }
}