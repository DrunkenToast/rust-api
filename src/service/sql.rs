use std::sync::Arc;

use rusqlite::Connection;
use rusqlite::params;
use tokio::sync::Mutex;
use crate::model::dht_measurement::DhtMeasurement;
use rusqlite::Result;

pub(crate) type DatabaseState = Arc<Mutex<Connection<>>>;

pub(crate) fn open_database_connection() -> Result<Connection> {
    let db = Connection::open("./db.db3")?;
    println!("Connected with db: {}", db.is_autocommit());
    init_db(&db)?;
    Ok(db)
}

fn init_db(con: &Connection) -> Result<()> {
    if let Err(e) = con.execute("
        CREATE TABLE IF NOT EXISTS dht (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            time DATETIME NOTE NULL,
            temperature DECIMAL(3, 1) NOT NULL,
            humidity DECIMAL(3, 1) NOT NULL
        )
    ", []) {
        panic!("{}", e);
    }
    Ok(())
}

impl DhtMeasurement {
    pub fn insert(&self, db: &Connection) -> Result<()> {
        let mut stmt = db.prepare_cached("INSERT INTO dht (time, temperature, humidity) VALUES (?1, ?2, ?3)")?;
        stmt.execute(params![self.time, self.temperature, self.humidity])?;
        Ok(())
    }

    pub fn select_all(db: &Connection) -> Result<Vec<Self>> {
        let mut stmt = db.prepare_cached("SELECT time, temperature, humidity FROM dht")?;
        let mut measurements = Vec::new();
        let dht_iter = stmt.query_map([], |row| {
            Ok(Self {
                time: row.get(0)?,
                temperature: row.get(1)?,
                humidity: row.get(2)?,
            })
        }).unwrap();
        for measurement in dht_iter {
            measurements.push(measurement?);
        }
        Ok(measurements)
    }
}