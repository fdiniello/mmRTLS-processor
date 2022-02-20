// pub mod multiple_measures;
mod beacon_measure;
mod known_position;

// Renaming types for ease of use outside the scope of this module

pub type BeaconMeasure = beacon_measure::BeaconMeasure;
pub type KnownPosition = known_position::KnownPosition;
