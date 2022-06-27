use crate::model::{health::HealthStatus, dht_measurement::DhtMeasurement};
use super::{Arduino, ArduinoError, Action};

impl Arduino {
    pub async fn health(&mut self) -> Result<HealthStatus, ArduinoError> {
        self.write_action(Action::Hello)?;
        match Action::try_from(self.read_or_timeout().await?) {
            Ok(Action::Recv) => {
                Ok(HealthStatus::Up)
            }
            _ => {
                Ok(HealthStatus::Down)
            }
        }
    }

    pub async fn switch_led(&mut self, state: bool) -> Result<(), ArduinoError> {
        self.write_action(Action::SwitchLed)?;
        println!("Switching led to {:?}", state as u8);
        self.write_u8(state as u8).ok();
        self.read_or_timeout().await?;
        Ok(())
    }

    pub async fn measure_dht(&mut self) -> Result<DhtMeasurement, ArduinoError>{
        println!("Length: {}", self.rx.len());
        println!("1");
        self.write_action(Action::ReadDHT)?;

        println!("2");
        // Read temperature
        let t1: i16 = dbg!(self.read_or_timeout().await)? as i16;
        let t2: i16 = dbg!(self.read_or_timeout().await)? as i16;
        let temperature: f64 = (t2 << 8 | t1) as f64 / 10.0;

        println!("3");
        // Read humidity
        let h1: u16 = dbg!(self.read_or_timeout().await)? as u16;
        let h2: u16 = dbg!(self.read_or_timeout().await)? as u16;
        let humidity: f64 = (h2 << 8 | h1) as f64 / 10.0;

        println!("4");
        dbg!(self.read_or_timeout().await)?;
        println!("5");

        Ok(DhtMeasurement {
            temperature,
            humidity,
        })
    }

    // Displays a message on the LCD screen of the arduino
    // Will first write a display message action, then the amount of bytes (clamped to 32 bytes).
    // After that the message itself is written.
    pub async fn display_message(&mut self, message: &str) -> Result<(), ArduinoError> {
        let message = message.as_bytes();
        let message = &message[..31.clamp(0, message.len())];
        if self.write_action(Action::DisplayMessage).is_ok() {
            self.serial.write(&[message.len() as u8]).ok();
            self.serial.write(message).ok();
            self.read_or_timeout().await?;
            Ok(())
        }
        else {Err(ArduinoError::IoError)}
    }
}