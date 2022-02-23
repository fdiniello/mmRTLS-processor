// pub mod multiple_measures;
mod beacon_measure;
mod device_status;
mod known_position;

// Renaming types for ease of use outside the scope of this module
pub const BEACONMEASURE_TIME_WINDOW: u64 = 4;
pub type BeaconMeasure = beacon_measure::BeaconMeasure;
pub type KnownPosition = known_position::KnownPosition;
pub type DeviceStatus = device_status::DeviceStatus;
