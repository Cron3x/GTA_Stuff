use std::{process::Command, fs::File, io::{Write, copy}, thread::Builder};
use eframe::{epi, egui::{self, Vec2, Label, Button, Window, Align}};

#[derive(Default)]

struct MyEguiApp {
	cont_btn_bool: bool,
}

impl epi::App for MyEguiApp {
	fn name(&self) -> &str {
		"Setup Window"
	}

	fn setup(&mut self, _ctx: &egui::CtxRef, _frame: &epi::Frame, _storage: Option<&dyn epi::Storage>) {
	}

	fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("You need Python 3.10 or above to run the scripts\n The Script will allso install all dependencies \n\n\nNeeded Packages:");
			ui.label("	- ip2geotools 								  	[ To localization the IP ]");
			ui.label("	- scapy													[ Analyze the Network Traffic ]");
			
			ui.label("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
			ui.horizontal(|ui| {
				let mut aw =  ui.add(Button::new(""));
				
				//ui.add(Stroke::new(1.0, Color32::LIGHT_BLUE));

				ui.add_sized((250.,0.), Label::new(" "));
				if ui.button( "Download Page").clicked() {
					Command::new("powershell ").arg("start").arg("https://www.python.org/").spawn().expect("msg");
				}

				let awd = Window::new("")
						.vscroll(false)
						.collapsible(false)
						.title_bar(false)
						.resizable(false);

				let text = if self.cont_btn_bool {
					"Working"
				} else {
					"Continue"
				};
				if ui.button(text).clicked() {
					self.cont_btn_bool = !self.cont_btn_bool;
				}
				if text == "Working"{
					install_python_dep().expect("can't install python dependencies");
					download_content().expect("can't download rest of the program");
					self.cont_btn_bool = !self.cont_btn_bool;
				}
			});
		});
	}
}

fn install_python_dep() -> std::io::Result<()>{
	let mut file = File::create("requirements.txt")?;
    file.write_all(b"ip2geotools\nscapy")?;
	let mut cmd = Command::new("pip3").args(["install", "-r", "requirements.txt"]).spawn().expect("msg");
	cmd.wait()?;
	Ok(())
}

fn download_content() -> std::io::Result<()>{

	let dlink = "https://gist.githubusercontent.com/Cron3x/e016e82caa9618546ab8366d51afac06/raw/524cd3a49cfabde5d1f75338438aedef798d3a95/my%2520GTA%2520suff%2520download%2520link";

	let mut aw = Command::new("powershell").arg("-Command").arg("(New-Object Net.WebClient).DownloadFile('https://www.dropbox.com/s/lxyp1104buf0iaj/gta_stuff.zip?dl=1', 'package.zip')").spawn()?;
	aw.wait()?;
	//TODO: unzip package.zip
	Ok(())
}


pub fn main() {
	let app = MyEguiApp::default();
	let mut native_options = eframe::NativeOptions::default();
	native_options.initial_window_size = Some(Vec2::new(444., 444.));
	eframe::run_native(Box::new(app), native_options);
}