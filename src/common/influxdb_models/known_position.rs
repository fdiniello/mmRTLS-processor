use chrono::{DateTime, Utc};
use influxdb::{InfluxDbWriteable, ReadQuery};
use serde::{Deserialize, Serialize};

use crate::helper::for_async::get_influx_cli;
use crate::{Point};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, InfluxDbWriteable)]
pub struct KnownPosition {
    pub x: f64,
    pub y: f64,
    pub time: DateTime<Utc>,
}

impl KnownPosition {
    #[allow(non_snake_case)]
    pub fn new(pos: Point) -> KnownPosition {
        KnownPosition {
            x: pos.x,
            y: pos.y,
            time: chrono::Utc::now(),
        }
    }
    pub async fn write_for(self, device_id: &str) -> Result<String, influxdb::Error> {
        let table_name = format!("position_{}", device_id);
        get_influx_cli()
            .query(self.into_query(table_name.as_str()))
            .await
    }
}


