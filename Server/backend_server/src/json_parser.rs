use serde_json::Value;

pub fn parse_sensor_data(json_str: &str) -> Result<(f64, f64, f64), &str> {
    // Parse the JSON string into a serde_json::Value
    let parsed: Value = serde_json::from_str(json_str).unwrap();

    // Extract each field manually
    let temperature = parsed
        .get("temperature")
        .ok_or("Missing 'temperature' field")?
        .as_f64()
        .ok_or("Invalid type for 'temperature'")?;

    let pressure = parsed
        .get("pressure")
        .ok_or("Missing 'pressure' field")?
        .as_f64()
        .ok_or("Invalid type for 'pressure'")?;

    let humidity = parsed
        .get("humidity")
        .ok_or("Missing 'humidity' field")?
        .as_f64()
        .ok_or("Invalid type for 'humidity'")?;

    Ok((temperature, pressure, humidity))
}