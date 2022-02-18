pub mod device {
    use common::influxdb_models::BeaconMeasure;
    use common::{DeviceReport, UnitsConvertion};

    pub async fn report(device_id: &str, payload: &str) {
        if let Ok(device_report) = serde_json::from_str::<DeviceReport>(payload) {
            // device_report.data.sort_by(|a, b| b.pwr.cmp(&a.pwr));

            for beacon in device_report.data.iter() {
                let measure = BeaconMeasure::new(
                    std::str::from_utf8(&beacon.id).unwrap_or("unknown"),
                    beacon.rssi.dBm_to_W(),
                );
                measure.write_for(device_id).await;
            }
        } else {
            println!("Unable to parse: {}", payload);
        }
    }
}
