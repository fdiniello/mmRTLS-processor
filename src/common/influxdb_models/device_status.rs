use influxdb::{ReadQuery, WriteQuery};
use serde::{Deserialize, Serialize};

use crate::{helper::for_async::get_influx_cli, MAC};

const TABLE_NAME: &str = "device_status";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceStatus {
    device_id: MAC,
    pos_x: f64,
    pos_y: f64,
    error: f64,
    speed: f64,
}

impl DeviceStatus {
    fn new(device_id: MAC) -> DeviceStatus {
        DeviceStatus {
            device_id,
            pos_x: 0.0,
            pos_y: 0.0,
            error: 0.0,
            speed: 0.0,
        }
    }

    pub async fn get(device_id: MAC) -> Result<Box<DeviceStatus>, influxdb::Error> {
        let query = ReadQuery::new(format!(
            "SELECT last(*) FROM /{}/ WHERE device_id = '{}';",
            TABLE_NAME, device_id
        ));
        let mut database_result = get_influx_cli().json_query(query).await?;

        #[derive(Debug, Deserialize)]
        struct Value {
            last_pos_x: f64,
            last_pos_y: f64,
            last_error: f64,
            last_speed: f64,
        }

        let vec = database_result.deserialize_next::<Value>()?.series;

        if vec.len() > 0 && vec[0].values.len() > 0 {
            Ok(Box::new(DeviceStatus {
                device_id,
                pos_x: vec[0].values[0].last_pos_x,
                pos_y: vec[0].values[0].last_pos_y,
                error: vec[0].values[0].last_error,
                speed: vec[0].values[0].last_speed,
            }))
        } else {
            Ok(Box::new(DeviceStatus::new(device_id)))
        }
    }

    fn as_query(&self) -> influxdb::WriteQuery {
        WriteQuery::new(influxdb::Timestamp::from(chrono::Utc::now()), TABLE_NAME)
            .add_tag("device_id", self.device_id)
            .add_field("pos_x", self.pos_x)
            .add_field("pos_y", self.pos_y)
            .add_field("error", self.error)
            .add_field("speed", self.speed)
    }

    async fn update(query: influxdb::WriteQuery) -> Result<String, influxdb::Error> {
        println!("update");
        get_influx_cli().query(query).await
    }
}

impl Drop for DeviceStatus {
    fn drop(&mut self) {
        println!("drop");
        let query = self.as_query();

        tokio::runtime::Handle::current().spawn(async move { Self::update(query).await });
    }
}

#[tokio::test]
async fn test() {
    use std::time::Duration;
    // create context to call drop
    {
        let mut a = DeviceStatus::get(MAC::new("15:23:45:ab:cd:ef"))
            .await
            .unwrap();
        a.pos_x += 2.0;
        a.pos_y += 3.0;
        println!("{:?}", a);
    } //here and then wait
    tokio::time::sleep(Duration::from_millis(150)).await;
}
