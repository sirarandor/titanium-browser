mod ti_track;
mod ti_depot;
pub fn railyard_init() {
	println!("Railyard Ran!");
}

pub fn debug() {
	ti_depot::ctrl::init();
	ti_track::ti_trainwreck::trainwreck_init();
	ti_track::track_init();
}