#![windows_subsystem = "windows"]
use crate::data_base;

use std::{thread::{self}, process::{Command}, os::windows::process::CommandExt};
use eframe::{run_native, epi::App, egui::{CentralPanel, ScrollArea, Vec2, Label, Button}, NativeOptions};

static mut TOGGLE_LOGGING:bool = false;

struct Headlines{
	articles: Vec<NewsCardData>,
}

impl Headlines {
	fn new() -> Headlines{

		let data = data_base::read("ips").expect("error !");
		let iter = (0..data.len()).map(|a| NewsCardData{
			header: format!("-------------------------<{}>-------------------------", a+1),
			ip: format!("IP: {}", data[&a]["ip"]),
			location: format!("Location: {}", data[&a]["location"]),
		});
		Headlines {
			articles: Vec::from_iter(iter),
		}
	}

	// fn configure_fonts(&self, ctx: &CtxRef) {
	// 	let mut font_def = FontDefinitions::default();
	// 	//font_def.font_data.insert("MesloLGS".to_string(), Cow::Borrowed(include_bytes!("../assets/fonts/MesloLGSNF.ttf")));
	// 	font_def.family_and_size.insert(eframe::egui::TextStyle::Heading, (FontFamily::Proportional, 35.));
	// 	font_def.family_and_size.insert(eframe::egui::TextStyle::Body, (FontFamily::Proportional, 20.));
	// 	font_def.fonts_for_family.get_mut(&FontFamily::Proportional).unwrap().insert(0, "MesloLGS".to_string());
	// 	ctx.set_fonts(font_def);
	// }
}

struct NewsCardData{
	header: String,
	ip: String,
	location: String,
}

impl App for Headlines {
	fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &eframe::epi::Frame) {
		CentralPanel::default().show(ctx, |ui| {
			ui.horizontal(|ui| {

				unsafe {
					let toggle_ip_listing_btn_txt = if TOGGLE_LOGGING{
						"Stop Listener"
					} else {
						"Start Listener"
					};
				
					let toggle_ip_listing_btn = ui.add_sized((300.0, 20.0), Button::new(toggle_ip_listing_btn_txt));	//TODO: Selectable Label
					if toggle_ip_listing_btn.clicked() {
						TOGGLE_LOGGING = !TOGGLE_LOGGING;
						sniffing_thread()
					}
				}
				let renew_ips = ui.add_sized((100.0, 20.0), Button::new("Delete IP list"));
				let save_to_file = ui.add_sized((100.0, 20.0), Button::new("Save To CSV File"));
				
				if renew_ips.clicked(){
					data_base::clear("ips").expect("Can't clear tabel ips");
				}
				if save_to_file.clicked(){
					//TODO: save current list to csv file
				}
			});
			ScrollArea::vertical().show(ui, |ui|{
				ui.add_sized((540.0, 0.0), Label::new(""));
				let arti = Headlines::new();
				for a in &arti.articles{
					ui.label(&a.header);
					ui.label(&a.ip);
					let beautyfied_location_a = format!("{}", a.location.replace("*", " "));
					let beautyfied_location_b = format!("{}", beautyfied_location_a.replace("+", ", "));
					ui.hyperlink_to(&beautyfied_location_b, format!("https://www.google.de/maps/search/{}", &beautyfied_location_b.replace("Location: ","")));
					ui.label("");
				}
			});
		});
	}

	fn name(&self) -> &str {
		"GTA IP Grabber"	
	}

	fn setup(
		&mut self,
		ctx: &eframe::egui::CtxRef,
		_frame: &eframe::epi::Frame,
		_storage: Option<&dyn eframe::epi::Storage>,
	) {
		//self.configure_fonts(ctx);
	}
	fn on_exit(&mut self) {
		unsafe {
			TOGGLE_LOGGING = false;
		}
	}
}


fn sniffing_thread(){
	unsafe {
		if TOGGLE_LOGGING {
			thread::spawn(move ||{
				while TOGGLE_LOGGING == true {
					let mut cmd = Command::new("python").arg("scripts/ip_grabber.pyw")
						.creation_flags(winapi::um::winbase::CREATE_NO_WINDOW)
						.spawn()
						.expect("--------------------------------------------------------------------------------------------------------------------------------");
					cmd.wait().expect("command wasn't running");
				}
			});
		}
	}
}

pub fn main() {

	data_base::create_table("ips").expect("could not create table");

	let app = Headlines::new();
	let mut win_options = NativeOptions::default();
	win_options.always_on_top = true;
	win_options.initial_window_size = Some(Vec2::new(540., 960.));	

	run_native(Box::new(app), win_options);
}
