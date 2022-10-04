mod ti_track;
mod ti_depot;

pub fn railyard_init() {
	println!("Railroad Ran!");
}

pub fn debug() {
	ti_depot::depot_init();
	ti_track::track_init();
	ti_track::ti_trainwreck::trainwreck_init();
}