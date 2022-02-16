use common::point::Point;

#[derive(Debug, Default)]
pub struct Antenna {
    pub id: [u8; 17],
    pub tssi: f64,
    pub coord: Point,
}

impl Antenna {
    pub fn new(id: &str, tssi: f64, coord: Point) -> Antenna {
        let mut a: Antenna = Default::default();
        a.id.copy_from_slice(id.as_bytes());
        a.tssi = tssi;
        a.coord = coord;
        a
    }

    pub fn get_rssi(&self, distance: f64) -> f64 {
        use std::f64::consts::PI;
        const C: f64 = 3e8;
        const F: f64 = 2.4e9;
        const K: f64 = C / F;

        let rssi = self.tssi - 2.0 * f64::log10(K / (distance * 4.0 * PI));
        rssi
    }
}
