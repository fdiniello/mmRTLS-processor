use rand_distr::{Distribution, Normal};
use std::{thread, time};

mod error_report;

use common::helper::for_sync::{get_mqtt_cli, mqtt_pub};
use common::{
    influxdb_models::{BeaconMeasure, KnownPosition},
    Antenna, DeviceReport, Point,
};

#[derive(Clone)]
pub struct Config {
    period_ms: u64,
    radius: f64,
    noise_level: f64,
    angle_step: f64,
    id: String,
    real: bool,
}

#[tokio::main]
async fn main() {
    let config = parse_cli();
    let period = time::Duration::from_millis(config.period_ms);
    let noise_gen = Normal::new(0.0, config.noise_level).unwrap();

    if config.real {
        let config = config.clone();
        tokio::spawn(async move {
            let _r = error_report::thread(config).await;
        });
    }

    let client = get_mqtt_cli();

    let mut position = Point::new(config.radius, 0.0);

    let antenna = vec![
        Antenna::new("e6:ad:0b:2e:d7:11", 30.0, Point::new(15.0, 15.0)),
        Antenna::new("c2:b5:f5:cc:e6:88", 30.0, Point::new(15.0, -15.0)),
        Antenna::new("e6:2e:e6:88:f5:cc", 30.0, Point::new(-15.0, 15.0)),
        Antenna::new("c2:ad:0b:b5:11:d7", 30.0, Point::new(-15.0, -15.0)),
    ];

    let topic = format!("device/{}/report", config.id);
    loop {
        let start = time::Instant::now();

        let mut report = DeviceReport { data: vec![] };

        for ant in (&antenna).into_iter() {
            let d = ant.coord.distance_to(&position);
            let rssi = ant.get_rssi(d);

            let noise: f64 = noise_gen.sample(&mut rand::thread_rng());

            report.data.push(BeaconMeasure::new(&ant.id, rssi + noise));
        }
        let payload = serde_json::to_string(&report).unwrap_or("".to_string());
        mqtt_pub(&client, topic.as_str(), payload.as_str()).expect("Pub error");

        if config.real {
            let _r = KnownPosition::new(position.clone()).write_for("real").await;
        }

        position.rotate_by(f64::to_radians(config.angle_step));
        thread::sleep(period - start.elapsed());
    }
}
fn parse_cli() -> Config {
    use std::env;
    let mut config = Config {
        period_ms: 1000,
        radius: 12.0,
        noise_level: 0.0,
        angle_step: 3.6,
        id: "60:f2:62:01:a9:28".to_string(),
        real: true,
    };

    let args = env::args().collect::<Vec<String>>();

    for (i, arg) in args.iter().enumerate() {
        match arg.as_str() {
            "--noise" | "--noise-level" | "-n" => {
                config.noise_level = args[i + 1].parse::<f64>().unwrap();
            }
            "--rad" | "--radious" | "-r" => {
                config.radius = args[i + 1].parse::<f64>().unwrap();
            }
            "--period" | "-p" => {
                config.period_ms = args[i + 1].parse::<u64>().unwrap();
            }
            "--angle" | "--step" => {
                config.angle_step = args[i + 1].parse::<f64>().unwrap();
            }
            "--id" => {
                config.id = args[i + 1].clone();
                config.real = false;
            }
            _ => {}
        }
    }
    config
}
