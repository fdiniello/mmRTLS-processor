use serde::{Deserialize, Serialize};

type MAC = [u8; 17];

#[derive(Debug, Serialize, Deserialize)]
pub struct Beacon {
    pub id: MAC,
    pub rssi: f64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceReport {
    pub data: Vec<Beacon>,
}
