use chrono::{DateTime, Utc};
use influxdb::{InfluxDbWriteable, ReadQuery};
use serde::{Deserialize, Serialize};

use crate::helper::for_async::get_influx_cli;
use crate::MAC;

pub const TIME_WINDOW: u64 = 4;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, InfluxDbWriteable)]
pub struct BeaconMeasure {
    #[influxdb(tag)]
    pub beacon_id: MAC,
    pub rssi: f64,
    pub time: DateTime<Utc>,
}

#[derive(Deserialize, Clone)]
pub struct BeaconMeasureWStats {
    pub beacon_id: MAC,
    pub time: DateTime<Utc>,
    pub mean: f64,
    pub stddev: f64,
    pub error: f64,
}

#[derive(Serialize, Deserialize)]
struct Tags {
    beacon_id: MAC,
}

impl BeaconMeasure {
    #[allow(non_snake_case)]
    pub fn new(beacon_id: &MAC, rssi_W: f64) -> BeaconMeasure {
        BeaconMeasure {
            beacon_id: beacon_id.clone(),
            rssi: rssi_W,
            time: chrono::Utc::now(),
        }
    }
    pub async fn write_for(self, device_id: &str) -> Result<String, influxdb::Error> {
        let table_name = format!("measure_{}", device_id);
        get_influx_cli()
            .query(self.into_query(table_name.as_str()))
            .await
    }
    pub async fn get_last_for(device_id: &str) -> Result<Vec<BeaconMeasure>, influxdb::Error> {
        let query = format!( "SELECT mean(rssi) FROM /measure_{}/ WHERE time > now() - {}s AND time < now() GROUP BY beacon_id;", device_id, TIME_WINDOW);

        let mut database_result = get_influx_cli().json_query(ReadQuery::new(query)).await?;

        #[derive(Deserialize)]
        struct Value {
            time: DateTime<Utc>,
            mean: f64,
        }
        let vect = database_result
            .deserialize_next_tagged::<Tags, Value>()?
            .series
            .into_iter()
            .map(|measure| BeaconMeasure {
                beacon_id: measure.tags.beacon_id,
                rssi: measure.values[0].mean,
                time: measure.values[0].time,
            })
            .collect::<Vec<BeaconMeasure>>();
        Ok(vect)
    }

    pub async fn get_full_stats_for(
        device_id: &str,
    ) -> Result<Vec<BeaconMeasureWStats>, influxdb::Error> {
        let query = format!( "SELECT mean(rssi), stddev(rssi), stddev(rssi)/mean(rssi) as error FROM /measure_{}/ WHERE time > now() - {}s AND time < now() GROUP BY beacon_id;", device_id, TIME_WINDOW);

        let mut database_result = get_influx_cli().json_query(ReadQuery::new(query)).await?;

        #[derive(Deserialize)]
        struct Value {
            time: DateTime<Utc>,
            mean: f64,
            stddev: f64,
            error: f64,
        }

        let vect = database_result
            .deserialize_next_tagged::<Tags, Value>()?
            .series
            .into_iter()
            .map(|measure| BeaconMeasureWStats {
                beacon_id: measure.tags.beacon_id,
                time: measure.values[0].time,
                mean: measure.values[0].mean,
                stddev: measure.values[0].stddev,
                error: measure.values[0].error,
            })
            .collect::<Vec<BeaconMeasureWStats>>();
        Ok(vect)
    }
}
