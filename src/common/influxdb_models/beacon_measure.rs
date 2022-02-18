use chrono::{DateTime, Utc};
use influxdb::{InfluxDbWriteable,ReadQuery};
use serde::Deserialize;

use crate::helper::for_async::get_influx_cli;


#[derive(Deserialize)]
struct Tags{
    beacon_id: String,
}

#[derive(Debug, PartialEq, Clone, InfluxDbWriteable)]
pub struct BeaconMeasure {
    #[influxdb(tag)]
    pub beacon_id: String,
    pub rssi: f64,
    pub time: DateTime<Utc>,
}

impl BeaconMeasure {
    #[allow(non_snake_case)]
    pub fn new( beacon_id: &str, rssi_W: f64) -> BeaconMeasure {
        BeaconMeasure {
            beacon_id: beacon_id.into(),
            rssi: rssi_W,
            time: chrono::Utc::now(),
        }
    }
    pub async fn write_for(self, device_id: &str) {
        let table_name = format!("measure_{}",device_id);
        let result = get_influx_cli()
            .query(self.into_query(table_name.as_str()))
            .await;
        if result.is_err() {
            print!("InfluxDB error when writing");
        }
    }
    pub async fn get_last_for(device_id: &str ) -> Result<Vec<BeaconMeasure>,influxdb::Error>{
        let query = format!( "SELECT last(\"rssi\") FROM \"db0\"..\"measure_{}\" WHERE \"time\" > now() - 2s AND \"time\" < now() GROUP BY \"beacon_id\";", device_id);

        let mut database_result = get_influx_cli().json_query( ReadQuery::new(query)).await?;

        #[derive(Deserialize)]
        struct  Value{
            time: DateTime<Utc>,
            last: f64,
        }
        let vect = database_result.deserialize_next_tagged::<Tags,Value>()?
            .series
            .into_iter()
            .map( | measure | {
                BeaconMeasure{
                    beacon_id: measure.tags.beacon_id,
                    rssi: measure.values[0].last,
                    time: measure.values[0].time,
                }
            })
            .collect::<Vec<BeaconMeasure>>();
        Ok(vect)               
    }
}