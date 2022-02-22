use chrono::{DateTime, Utc};
use common::helper::for_async::get_influx_cli;
use influxdb::InfluxDbWriteable;
use serde::Serialize;
use tokio::time;

use common::{influxdb_models::KnownPosition, Point};

use crate::Config;

#[derive(Debug, Serialize, InfluxDbWriteable)]
pub struct Error {
    error: f64,
    speed: f64,
    time: DateTime<Utc>,
}

pub async fn thread(config: Config) {
    let period = time::Duration::from_millis(500);

    let mut position = Point::new(config.radius, 0.0);
    let mut speed = position.clone();
    position.rotate_by(f64::to_radians(config.angle_step));
    speed -= position;

    let speed = speed.module();

    loop {
        let start = time::Instant::now();

        let real = KnownPosition::get_last_for("real", 1).await;
        let calc = KnownPosition::get_last_for(config.id.as_str(), 1).await;
        if real.is_ok() && calc.is_ok() {
            let real = real.unwrap();
            let calc = calc.unwrap();

            if real.is_some() && calc.is_some() {
                let real = real.unwrap();
                let calc = calc.unwrap();
                #[allow(non_snake_case)]
                let Δx = real.x - calc.x;
                #[allow(non_snake_case)]
                let Δy = real.y - calc.y;
                let error = Error {
                    speed: speed,
                    error: f64::sqrt(Δx.powi(2) + Δy.powi(2)),
                    time: chrono::Utc::now(),
                };

                let table_name = format!("error_{}", config.id.as_str());
                get_influx_cli()
                    .query(error.into_query(table_name.as_str()))
                    .await
                    .unwrap();
            }
            time::sleep(period - start.elapsed()).await;
        }
    }
}
