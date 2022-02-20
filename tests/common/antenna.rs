use common::Antenna;
use common::Point;

#[test]
fn antenna_test() {
    let tssi = 0.0; // dBm
    let a = Antenna::new("AB:CD:EF:12:34:56", tssi, Point { x: 0.0, y: 0.0 });

    // Known Attenuation values for 2.4GHz
    //  5 meter = 54.02 dB = 3.96e-9 W
    // 10 meter = 60.04 dB = 9.91e-10 W
    // 20 meter = 66.06 dB = 2.48e-10 W

    print!("Testing Antenna::get_rssi()");
    assert!(f64::abs(-54.02 - a.get_rssi(5.0)) < 0.1);
    assert!(f64::abs(-60.04 - a.get_rssi(10.0)) < 0.1);
    assert!(f64::abs(-66.06 - a.get_rssi(20.0)) < 0.1);
    println!(" ... ok");

    print!("Testing Antenna::get_distance_with_dBm()");
    assert!(f64::abs(5.0 - a.get_distance_with_dBm(-54.02)) < 0.5);
    assert!(f64::abs(10.0 - a.get_distance_with_dBm(-60.04)) < 0.5);
    assert!(f64::abs(20.0 - a.get_distance_with_dBm(-66.06)) < 0.5);
    println!(" ... ok");

    print!("Testing Antenna::get_distance_with_W()");
    assert!(f64::abs(5.0 - a.get_distance_with_W(3.98e-9)) < 0.5);
    assert!(f64::abs(10.0 - a.get_distance_with_W(9.91e-10)) < 0.5);
    assert!(f64::abs(20.0 - a.get_distance_with_W(2.48e-10)) < 0.5);
    println!(" ... ok");
}
