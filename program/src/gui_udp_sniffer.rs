//#![windows_subsystem = "windows"]
use crate::data_base;

use std::{thread::{self}, process::{Command}, os::windows::{process::CommandExt}, fs, io::Write};
use eframe::{run_native, epi::App, egui::{CentralPanel, ScrollArea, Vec2, Label, Button, self, Window, TextStyle}, NativeOptions};

static mut TOGGLE_LOGGING:bool = false;
static mut TOGGLE_SAVE_WINDOW:bool = false;

pub struct CSVWindow {
    seperator: String,
	headings: String,
}

impl CSVWindow {
    fn CSVWindow() -> Self {
        Self {
            seperator: ",".to_owned(),
			headings: "time , ip, location".to_string()
        }
    }
}


struct Headlines{
	articles: Vec<NewsCardData>,
	cont_btn_bool: bool,
}

impl Headlines {
	fn new() -> Headlines{

		let data = data_base::read("ips").expect("error !");
		let iter = (0..data.len()).map(|a| NewsCardData{
			header: format!("-------------------------<{}>-------------------------", a+1),
			ip: format!("IP: {}", data[&a]["ip"]),
			location: format!("Location: {}", data[&a]["location"]),
			time: format!("Time: {}", data[&a]["time"])
		});
		Headlines {
			articles: Vec::from_iter(iter),
			cont_btn_bool: false,
		}
	}
}

struct NewsCardData{
	header: String,
	ip: String,
	location: String,
	time: String,
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

				unsafe {
					let csv_save_window = Window::new("Save to files")
						.anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
						.min_height(1000.)
						.resizable(true);

					if save_to_file.clicked(){
						TOGGLE_SAVE_WINDOW = !TOGGLE_SAVE_WINDOW;
					}
					
					if TOGGLE_SAVE_WINDOW == true {
						csv_save_window.show(ctx, |ui|{
							//ui.add_space(100.);
							ui.group(|ui| {
								ui.horizontal(|ui|{
									ui.vertical(|ui| {
										ui.horizontal(|ui|{
											ui.add_sized((10., 5.), Label::new("Seperator"));
											let mut csv_window_options = CSVWindow::CSVWindow();
											ui.add_space(10.);
											ui.add_sized((5., 5.), egui::TextEdit::singleline(&mut csv_window_options.seperator).code_editor());
										});
										ui.add_space(100.)
									});

									ui.vertical(|ui| {
										ScrollArea::vertical()
										.show(ui, |ui| {
											ui.set_height(100.);
											
											ui.add(egui::TextEdit::multiline(&mut format_csv()));
										});
									});
								});	
							});

							ui.add_space(20.);
							ui.group(|ui| {
								ui.horizontal(|ui| {
									let c_btn = Button::new("Cancel");
									let d_btn = Button::new("Fill with DbugData");
									let s_btn = Button::new("Save");
									if ui.add_enabled(true, c_btn).clicked() {
										TOGGLE_SAVE_WINDOW = !TOGGLE_SAVE_WINDOW;
									}
									ui.add_space(110.);
									if ui.add_enabled(true, d_btn
									).clicked() {
										data_base::fill_with_dummy_data( 10).expect("Can't create Dummy Data");
									}
									ui.add_space(110.);
									if ui.add_enabled(true, s_btn).clicked() {
										
										fs::create_dir_all("./csv").expect("Can't create folder");
										let mut csv_file = fs::File::create("./csv/ips.csv").expect("Can't create file");

										let format_csv = format_csv();

										csv_file.write_all(format_csv.as_bytes()).expect("Can't write to csv file");

										Command::new("explorer").arg(".\\csv\\").spawn().expect("Can't open explorer"); //üüü
									}
								});

								
								// ui.available_height();
								// ui.available_width();
							});
						});
					}
				}
					
			});
			ScrollArea::vertical().show(ui, |ui|{
				ui.add_sized((540.0, 0.0), Label::new(""));
				let arti = Headlines::new();
				for a in &arti.articles{
					ui.label(&a.header);
					ui.label(&a.time);
					ui.label( &a.ip);

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

fn format_csv() -> String {
	let arti = Headlines::new();
	let mut mtext:String = CSVWindow::CSVWindow().headings;

	for a in &arti.articles{
	
		let cur_ip = a.time.as_str();
		let vip: Vec<&str> = cur_ip.split(":").collect();
		let mut _vip: Vec<&str> = vip[1].split("\"").collect();
		mtext = format!("{}\n{}", mtext ,_vip[1]);

		let cur_loc = a.ip.as_str();
		let vloc: Vec<&str> = cur_loc.split(":").collect();
		let mut _vloc: Vec<&str> = vloc[1].split("\"").collect();
		mtext = format!("{}, {}", mtext ,_vloc[1]);

		let cur_loc = a.location.as_str();
		let vloc: Vec<&str> = cur_loc.split(":").collect();
		let mut _vloc: Vec<&str> = vloc[1].split("\"").collect();
		mtext = format!("{}, {}", mtext ,_vloc[1]);
	}
	return mtext;
}


pub fn main() {

	data_base::create_table().expect("could not create table");

	let app = Headlines::new();
	let mut win_options = NativeOptions::default();
	win_options.always_on_top = true;
	win_options.initial_window_size = Some(Vec2::new(540., 960.));	

	run_native(Box::new(app), win_options);
}
