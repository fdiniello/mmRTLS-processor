// pub mod multiple_measures;
pub mod beacon_measure;
pub mod known_position;

// Renaming types for ease of use outside the scope of this module

pub type BeaconMeasure = beacon_measure::BeaconMeasure;
pub type KnownPosition = known_position::KnownPosition;
