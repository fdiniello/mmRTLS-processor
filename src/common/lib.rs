pub mod device_report;
pub mod helper;
pub mod point;

pub trait UnitsConvertion{
    #[allow(non_snake_case)]
    fn dBm_to_W(&self) -> f64;
    #[allow(non_snake_case)]
    fn W_to_dBm(&self) -> f64;
}

impl UnitsConvertion for f64 {
    fn dBm_to_W(&self) -> f64 {
        10.0_f64.powf( (self-30.0)/10.0)
    }
    fn W_to_dBm(&self) -> f64 {
        30.0 + 10.0 * f64::log10(*self)
    }
}