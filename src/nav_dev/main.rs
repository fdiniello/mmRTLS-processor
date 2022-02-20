use std::{thread, time};

use common::helper::for_sync::{get_mqtt_cli, mqtt_pub};
use common::{
    influxdb_models::{BeaconMeasure, KnownPosition},
    Antenna, DeviceReport, Point,
};

#[tokio::main]
async fn main() {
    let client = get_mqtt_cli();
    let period = time::Duration::from_millis(1000);

    let mut position = Point::new(12.0, 0.0);

    let antenna = vec![
        Antenna::new("e6:ad:0b:2e:d7:11", 30.0, Point::new(15.0, 15.0)),
        Antenna::new("c2:b5:f5:cc:e6:88", 30.0, Point::new(15.0, -15.0)),
        Antenna::new("e6:2e:e6:88:f5:cc", 30.0, Point::new(-15.0, 15.0)),
        Antenna::new("c2:ad:0b:b5:11:d7", 30.0, Point::new(-15.0, -15.0)),
    ];

    let topic = "device/60:f2:62:01:a9:28/report";
    loop {
        let start = time::Instant::now();

        let mut report = DeviceReport { data: vec![] };

        for ant in (&antenna).into_iter() {
            let d = ant.coord.distance_to(&position);
            let rssi = ant.get_rssi(d);

            let noise = 0.0;
            // let noise: f64 = 1.0 * rand::random::<f64>() - 0.5;

            report.data.push(BeaconMeasure::new(&ant.id, rssi + noise));
        }
        let payload = serde_json::to_string(&report).unwrap_or("".to_string());
        mqtt_pub(&client, topic, payload.as_str()).expect("Pub error");

        let _r = KnownPosition::new(position.clone()).write_for("real").await;

        position.rotate_by(f64::to_radians(3.6));
        thread::sleep(period - start.elapsed());
    }
}
