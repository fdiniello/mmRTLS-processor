use crate::{point::Point, UnitsConvertion};
use std::f64::consts::PI;

#[derive(Debug, Default)]
pub struct Antenna {
    pub id: String,
    pub tssi: f64,
    pub coord: Point,
}

impl Antenna {
    const C: f64 = 2.99e8;
    const F: f64 = 2.4e9;
    #[allow(non_upper_case_globals)]
    const λ: f64 = Self::C / Self::F;

    pub fn new<S1>(id: S1, tssi: f64, coord: Point) -> Antenna
    where
        S1: ToString,
    {
        Antenna {
            id: id.to_string(),
            coord: coord,
            tssi: tssi,
        }
    }

    pub fn get_rssi(&self, distance: f64) -> f64 {
        #[allow(non_snake_case)]
        // Free Space Path Loss
        let FSPL = (((distance * 4.0 * PI) / Self::λ).powi(2)).to_dB();
        let rssi = self.tssi - FSPL;
        rssi
    }
    #[allow(non_snake_case)]
    pub fn get_distance_with_dBm(&self, rssi_dBm: f64) -> f64 {
        let loss = self.tssi.dBm_to_W() / rssi_dBm.dBm_to_W();
        let distance = (loss.sqrt() * Self::λ) / (4.0 * PI);
        distance.abs()
    }
    #[allow(non_snake_case)]
    pub fn get_distance_with_W(&self, rssi_W: f64) -> f64 {
        let loss = self.tssi.dBm_to_W() / rssi_W;
        let distance = (loss.sqrt() * Self::λ) / (4.0 * PI);
        distance.abs()
    }
}
