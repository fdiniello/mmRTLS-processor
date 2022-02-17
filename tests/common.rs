use common::UnitsConvertion;

#[test]
fn test_unit_convertion() {
    print!("Testing converion from W to dBm");
    assert_eq!(1.0_f64.W_to_dBm(), 30.0);
    assert_eq!(0.001_f64.W_to_dBm(), 0.0);
    assert!(f64::abs(2.0_f64.W_to_dBm() - 33.0) < 0.1);
    assert!(f64::abs(0.002_f64.W_to_dBm() - 3.0) < 0.1);
    println!(" ... ok");

    print!("Testing converion from dBm to W");
    assert_eq!(1.0, 30.0_f64.dBm_to_W());
    assert_eq!(0.001, 0.0_f64.dBm_to_W());
    assert!(f64::abs(2.0 - 33.0_f64.dBm_to_W()) < 0.1);
    assert!(f64::abs(0.002 - 3.0_f64.dBm_to_W()) < 0.1);
    println!(" ... ok");

    print!("Testing converion from dB to scalar");
    assert_eq!(1.0, 0.0_f64.from_dB());
    assert_eq!(10.0, 10.0_f64.from_dB());
    assert_eq!(100.0, 20.0_f64.from_dB());
    assert!(f64::abs(2.0 - 3.0_f64.from_dB()) < 0.1);
    assert!(f64::abs(20.0 - 13_f64.from_dB()) < 0.1);
    assert!(f64::abs(200.0 - 23_f64.from_dB()) < 0.5);
    println!(" ... ok");

    print!("Testing converion from scalar to dB");
    assert_eq!(1.0.to_dB(), 0.0);
    assert_eq!(10.0.to_dB(), 10.0);
    assert_eq!(100.0.to_dB(), 20.0);
    assert!(f64::abs(2.0.to_dB() - 3.0) < 0.1);
    assert!(f64::abs(20.0.to_dB() - 13.0) < 0.1);
    assert!(f64::abs(200.0.to_dB() - 23.0) < 0.5);
    println!(" ... ok");
}
