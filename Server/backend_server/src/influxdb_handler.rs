use std::error::Error;

use influxdb::{Client, Timestamp};
use influxdb::InfluxDbWriteable;
use std::time::{SystemTime, UNIX_EPOCH};

/// Database writer struct
pub struct DbWriter {
    client: Client,
}

impl DbWriter {
    /// Create a new instance of DbWriter
    pub fn new(base_url: &str, database: &str) -> Self {
        let client = Client::new(base_url, database);
        DbWriter { client }
    }

    pub async fn write_temperature(&self, temperature: f64) -> Result<(), Box<dyn Error>> {
        let start = SystemTime::now();

        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        let query = Timestamp::Milliseconds(since_the_epoch)
            .into_query("temperature")
            .add_field("temperature", temperature);

        self.client.query(query).await?;

        Ok(())
    }

    pub async fn write_pressure(&self, pressure: f64) -> Result<(), Box<dyn Error>> {
        let start = SystemTime::now();

        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        let query = Timestamp::Milliseconds(since_the_epoch)
            .into_query("pressure")
            .add_field("pressure", pressure);
        
        self.client.query(query).await?;

        Ok(())
    }

    pub async fn write_humidity(&self, humidity: f64) -> Result<(), Box<dyn Error>> {
        let start = SystemTime::now();

        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        let query = Timestamp::Milliseconds(since_the_epoch)
            .into_query("humidity")
            .add_field("humidity", humidity);
        
        self.client.query(query).await?;

        Ok(())
    }
}
