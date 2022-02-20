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
    pub async fn get_last_for(device_id: &str) -> Result<Vec<KnownPosition>, influxdb::Error> {
        let query = format!(
            "SELECT x, y FROM /position_{}/ WHERE time > now() - 4s AND time < now();",
            device_id
        );

        let mut database_result = get_influx_cli().json_query(ReadQuery::new(query)).await?;

        let series = &database_result.deserialize_next::<KnownPosition>()?.series;
        if series.len() >= 1 {
            let vec = &series[0].values;
            Ok(vec.to_vec())
        } else {
            Ok(vec![])
        }
    }
}
