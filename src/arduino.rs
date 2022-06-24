use std::thread;
use std::time::Duration;
use std::{io::Write, sync::Arc};
use std::io::{self, Read};
use tokio_serial::{SerialPort, Parity};
use tokio::sync::Mutex;
use tokio::sync::broadcast::{self, Sender, Receiver};

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
    //rx: Receiver<u8>
}

impl Arduino {
    pub async fn new(port: String) -> Self {
        let (tx, mut rx): (Sender<u8>, Receiver<u8>) = broadcast::channel(16);
        let mut rx2 = tx.subscribe();
        let serial = tokio_serial::new(&port, 115200)
            .parity(Parity::None).timeout(Duration::from_millis(100))
            .open().expect("Failed to open port");
        let mut serial2 = serial.try_clone().expect("clone serial failed");

        let mut con = Self {
            serial: serial,
            //rx: rx2
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
            if let Ok(val) = rx2.recv().await{
                match Action::try_from(val) {
                    Ok(Action::Hello) => {
                        con.write_action(Action::Hello).unwrap();
                    },
                    Ok(Action::Recv) => {
                        rx2 = rx2.resubscribe();
                        break;
                    },
                    _ => {}
                }
            }
        }
        println!("Done?");
        con
    }

    fn write_action(&mut self, action: Action) -> Result<(), io::Error> {
        self.write_u8(action as u8)?;
        Ok(())
    }

    fn write_u8(&mut self, b: u8) -> Result<(), io::Error> {
        let buf = [b];
        self.serial.write(&buf)?;
        self.serial.flush()?;
        Ok(())
    }

    pub fn switch_led(&mut self, state: bool) {
        if let Ok(_) = self.write_action(Action::SwitchLed) {
            println!("Switching led to {:?}", state as u8);
            self.write_u8(state as u8).ok();
        }
        else {
            println!("???")
        }
    }

    // Displays a message on the LCD screen of the arduino
    // Will first write a display message action, then the amount of bytes (clamped to 32 bytes).
    // After that the message itself is written.
    pub fn display_message(&mut self, message: &str) {
        let message = message.as_bytes();
        let message = &message[..31.clamp(0, message.len())];
        if let Ok(_) = self.write_action(Action::DisplayMessage) {
            self.serial.write(&[message.len() as u8]).ok();
            self.serial.write(message).ok();
        }
    }
}