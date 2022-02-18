use chrono::{DateTime, Utc};
use influxdb::InfluxDbWriteable;

use crate::helper::for_async::get_influx_cli;

#[derive(Debug, InfluxDbWriteable)]
pub struct SingleMeasureWO<'a> {
    pub device_id: &'a str,
    #[influxdb(tag)]
    pub beacon_id: &'a str,
    pub rssi: f64,
    pub time: DateTime<Utc>,
}

impl<'a> SingleMeasureWO<'a> {
    #[allow(non_snake_case)]
    pub fn new( device_id: &'a str, beacon_id: &'a str, rssi_W: f64) -> SingleMeasureWO<'a> {
        SingleMeasureWO {
            device_id: device_id,
            beacon_id: beacon_id,
            rssi: rssi_W,
            time: chrono::Utc::now()
        }
    }
    pub async fn write(self) {
        let result = get_influx_cli()
            .query(self.into_query("measure"))
            .await;
        if result.is_err() {
            print!("InfluxDB error when writing");
        }
    }
}