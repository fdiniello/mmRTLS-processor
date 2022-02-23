use common::influxdb_models::BeaconMeasure;
use std::time::Duration;

use common::MAC;

#[tokio::test]
async fn beacon_measure_test() {
    print!("Testing BeaconMeasure::* read/write methods");
    let bm1 = BeaconMeasure::new(&MAC::new("AB:CD:EF:12:34:56"), 0.0);
    let bm = bm1.clone();
    let _result = bm.write_for("AB:CD:EF:12:34:56").await;

    let bm2 = BeaconMeasure::get_for("AB:CD:EF:12:34:56").await.unwrap();
    assert_eq!(bm2.len(), 1);
    assert_eq!(bm1.beacon_id, bm2[0].beacon_id);
    assert_eq!(bm1.rssi, bm2[0].rssi);

    //wait for the time window to pass
    let delay = common::influxdb_models::BEACONMEASURE_TIME_WINDOW * 1000 + 500;
    tokio::time::sleep(Duration::from_millis(delay)).await;
    let bm2 = BeaconMeasure::get_for("AB:CD:EF:12:34:56").await.unwrap();
    assert_eq!(bm2.len(), 0);

    println!(" ... ok");
}
