use common::influxdb_models::BeaconMeasure;
use std::time::Duration;

#[tokio::test]
async fn beacon_measure_test() {
    print!("Testing BeaconMeasure::* read/write methods");
    let bm1 = BeaconMeasure::new("test", 0.0);
    let bm = bm1.clone();
    bm.write_for("test").await;

    let bm2 = BeaconMeasure::get_last_for("test").await.unwrap();
    assert_eq!(bm2.len(), 1);
    assert_eq!(bm1, bm2[0]);

    //wait for the time window to pass
    tokio::time::sleep(Duration::from_millis(2100)).await;
    let bm2 = BeaconMeasure::get_last_for("test").await.unwrap();
    assert_eq!(bm2.len(), 0);

    println!(" ... ok");
}
