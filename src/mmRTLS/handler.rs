pub mod device {

    use chrono::{DateTime, Utc};

    use common::helper::for_async::get_influx_cli;
    use influxdb::InfluxDbWriteable;
    // use influxdb::{Query, ReadQuery, Timestamp};

    use common::device_report::DeviceReport;

    #[derive(Debug, InfluxDbWriteable)]
    struct Measure<'a> {
        #[influxdb(tag)]
        device_id: &'a str,
        #[influxdb(tag)]
        beacon_id: &'a str,
        rssi: f64,
        time: DateTime<Utc>,
    }
    pub async fn report(device_id: &str, payload: &str) {
        if let Ok(payload) = serde_json::from_str::<DeviceReport>(payload) {
            // payload.data.sort_by(|a, b| b.pwr.cmp(&a.pwr));

            let time_stamp: DateTime<Utc> = chrono::Utc::now();

            for b in payload.data.iter() {
                let event = Measure {
                    time: time_stamp,
                    device_id: device_id,
                    beacon_id: std::str::from_utf8(&b.id).unwrap_or("unknown"),
                    rssi: b.rssi,
                };
                get_influx_cli()
                    .query(event.into_query("measure"))
                    .await
                    .expect("influx query error");
            }
        } else {
            println!("Unable to parse: {}", payload);
        }
    }
}
