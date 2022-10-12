use eframe::egui;
use curl::easy::Easy;




pub mod ti_trainwreck;



pub fn track_init() {
	// These two variables need to be able to be accessed in the 
	// impl efram::App for TitaniumApp, in order to get the page.
	// 
	// I think a struct will work best, but... I'm out of knowledge
	// on this one. Whatever you think will work here in order to do this.
	// 
	// Note: They need to stay in scope constantly, as handle is
	// used for the screen, and address is used for what page we are loading.
	
	let mut handle = Easy::new();
	let mut address = "http://tryhrdsnphrd.com/titanium/test.html";
	println!("Track Initializing");
	
	// Set up the screen to run
	// This first one is the defaults, I figure it's good to let us change it
	let default_options = eframe::NativeOptions::default();
	// This second one just copies it, but lets us change whatever we want first
	let native_options = eframe::NativeOptions {
		//maximized:true,
		vsync:true,
		..default_options
	};
	// This tells the window to start running
	eframe::run_native("Titanium", native_options, Box::new(|cc| Box::new(TitaniumApp::new(cc))));
}

// The actual app itself
#[derive(Default)]
struct TitaniumApp {}

impl TitaniumApp {
	fn new(cc: &eframe::CreationContext<'_>) -> Self {
		// We can use this to customize EGUI to be what we want
		Self::default() // Return itself
		
	}
}

impl eframe::App for TitaniumApp {
	fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("Titanium is running!");
			// This currently errors, since address is out of scope here.
			ui.hyperlink(address);
		});
	}
}