
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

use tokio::time::sleep;
use tokio::time::Duration;

use tokio::task;
use tokio::io::{AsyncBufReadExt};

use core::usize;
use std::error::Error;

// local modules
mod json_parser;
mod influxdb_handler;

use json_parser::parse_sensor_data;
use influxdb_handler::DbWriter;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Bind the listener to a local address
    let listener = TcpListener::bind("0.0.0.0:5000").await?;

    // Accept incoming connections
    loop {
        // Accept a new connection
        let (stream, _) = listener.accept().await?;

        // Spawn a new asynchronous task to handle the connection
        task::spawn(async move {
            if let Err(e) = handle_connection(stream).await {
                eprintln!("Failed to handle connection: {:?}", e);
            }
        });
    }
}


async fn handle_connection(stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {

    let mut buf_reader = tokio::io::BufReader::new(stream);
    let mut headers = String::new();

    let mut content_length = 0;
    loop {
        let mut line = String::new();
        let _ = buf_reader.read_line(&mut line).await?;
        
        if line == "\r\n" {
            break;
        }

        if let Some(len) = line.strip_prefix("Content-Length: ") {
            content_length = len.trim().parse::<usize>().unwrap();
        }
        headers.push_str(&line);
    }

    let mut body = vec![0; content_length];
    buf_reader.read_exact(&mut body).await?;
    let body_str = String::from_utf8(body).unwrap();

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    buf_reader.get_mut().write_all(response.as_bytes()).await?;

    // parse json values
    let (temp, pres, hum) = parse_sensor_data(&body_str).unwrap();

    // write to influx database
    write_db_values(temp, pres, hum).await?;

    Ok(())
}

async fn write_db_values(temperature: f64, pressure: f64, humidity: f64) -> Result<(), Box<dyn Error>> {
    let db_writer = DbWriter::new("http://user:password@influxdb:8086", "sensor_database");

    db_writer.write_temperature(temperature).await?;
    db_writer.write_pressure(pressure).await?;
    db_writer.write_humidity(humidity).await?;

    Ok(())
}
