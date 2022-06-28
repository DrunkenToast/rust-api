use std::time::Duration;

use super::{arduino::ArduinoState, sql::DatabaseState};
use tokio::time;

pub(crate) fn start_scheduler(arduino: ArduinoState, db: DatabaseState) {
    let mut interval = time::interval(Duration::from_secs(5*60));
    tokio::spawn(async move {
        loop {
            interval.tick().await;
            println!("Running scheduler...");
            let mut arduino = arduino.lock().await;
            let db = db.lock().await;
            if let Ok(measurement) = arduino.measure_dht().await {
                measurement.insert(&db).expect("Insert failed");
            }
        }
    });
}