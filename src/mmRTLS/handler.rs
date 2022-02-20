pub mod device {
    use common::influxdb_models::BeaconMeasure;
    use common::{DeviceReport, UnitsConvertion,MAC};

    use crate::position_solver::solve_for;


    pub async fn report(device_id: &str, payload: &str) {
        if let Ok(device_report) = serde_json::from_str::<DeviceReport>(payload) {
            // device_report.data.sort_by(|a, b| b.pwr.cmp(&a.pwr));

            let mut count = 0;
            for beacon in device_report.data.iter() {
                let measure = BeaconMeasure::new(&beacon.beacon_id, beacon.rssi.dBm_to_W());
                if let Ok(_) = measure.write_for(device_id).await{
                    count+=1;
                }
            }

            // If I added more than 3 valid measures it's worth to process the position
            if count >= 3 {
                let device_id = MAC::new(device_id);
                tokio::spawn( async move {
                    let _r = solve_for( device_id ).await;
                });
            }

        } else {
            println!("Unable to parse: {}", payload);
        }
    }
}
