use common::UnitsConvertion;


#[test]
fn test_unit_convertion(){
    
    print!("Testing converion from W to dBm");
    assert_eq!( 1.0_f64.W_to_dBm() , 30.0 );
    assert_eq!( 0.001_f64.W_to_dBm(), 0.0 );
    assert!( 2.0_f64.W_to_dBm() - 33.0 < 0.1 );
    assert!( 0.002_f64.W_to_dBm() - 3.0 < 0.1 );
    println!(" ... ok");
    
    print!("Testing converion from dBm to W");
    assert_eq!( 1.0 , 30.0_f64.dBm_to_W() );
    assert_eq!( 0.001, 0.0_f64.dBm_to_W() );
    assert!( 2.0_f64 - 33.0_f64.dBm_to_W() < 0.1 );
    assert!( 0.002 - 3.0_f64.dBm_to_W() < 0.1 );
    println!(" ... ok");



}