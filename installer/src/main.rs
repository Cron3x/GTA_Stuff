#![windows_subsystem = "windows"]
use std::{process::{Command, exit}, fs::{File, self}, io::{Write, BufReader}};
use eframe::{epi, egui::{self, Vec2, Label, Button, Window}};
use zip::{ZipArchive};

#[derive(Default)]
struct MyEguiApp {
	cont_btn_bool: bool,
	add_label_bool:bool,
}

impl epi::App for MyEguiApp {
	fn name(&self) -> &str {
		"Setup Window"
	}

	fn setup(&mut self, _ctx: &egui::CtxRef, _frame: &epi::Frame, _storage: Option<&dyn epi::Storage>) {
	}

	fn on_exit(&mut self) {
		clean_files().expect("can't clean files");
	}

	fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("You need Python 3.10 or above to run the scripts\n This Installer will allso install all dependencies \n\n\nNeeded Python Packages:");
			ui.label("	- ip2geotools 								  	[ To localization the IP ]");
			ui.label("	- scapy													[ Analyze the Network Traffic ]");
			
			ui.label("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
			
			ui.horizontal(|ui| {			

				ui.group(|ui| {
					let start_listener = ui.add_enabled(self.add_label_bool, Button::new("Start Listener"));
					if start_listener.clicked() {
						Command::new("gta_stuff.exe").spawn().expect("Error Opening listener");
						exit(0);
					}
				});				

				ui.add_sized((132.,0.), Label::new(" "));

				let window = Window::new("")
						.vscroll(false)
						.collapsible(false)
						.title_bar(false)
						.resizable(false)
						.default_pos((175.,175.));

				

				let text = if self.cont_btn_bool {
					"Working"
				} else {
					"Continue"
				};

				ui.group(|ui| {
					if ui.button( "Download Python").clicked() {
						Command::new("powershell ").arg("start").arg("https://www.python.org/").spawn().expect("can't open Browser");
					}

					if ui.button(text).clicked() {
						self.cont_btn_bool = !self.cont_btn_bool;
						window.show(ctx, |ui|{
							ui.add_space(20.);
							ui.label("			Installing\n	 please stand by		");
							ui.add_space(20.);
						});
					}
				});

				if text == "Working"{
					download_content().expect("can't download rest of the program");
					self.cont_btn_bool = !self.cont_btn_bool;
					self.add_label_bool = true;
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
	println!("- Installing PIP Packages -");
	install_python_dep().expect("can't install python dependencies");
	println!("- Installing Finished -");
	println!("- Start Download -");
	let mut dchild = Command::new("powershell").arg("-Command").arg("(New-Object Net.WebClient).DownloadFile('https://www.dropbox.com/s/lxyp1104buf0iaj/gta_stuff.zip?dl=1', 'package.zip')").spawn()?;
	dchild.wait()?;
	println!("- Download Finished");
	println!("- Extracting Files -");
	let f = File::open("package.zip")?;
    let reader = BufReader::new(f);
	ZipArchive::extract(&mut ZipArchive::new(reader).unwrap(),"").expect("Can't extract package.zip");
	println!("- Extracting Finished -");
	clean_files().expect("can't clean files");
	Ok(())
}

fn clean_files() -> std::io::Result<()>{
	fs::remove_file("requirements.txt")?;
	fs::remove_file("package.zip")?;
    Ok(())
}

fn hide_console_window() {
    unsafe { winapi::um::wincon::FreeConsole() };
}
// 
fn main() {
	hide_console_window();
	let app = MyEguiApp::default();
	let mut native_options = eframe::NativeOptions::default();
	native_options.initial_window_size = Some(Vec2::new(444., 444.));
	native_options.always_on_top = true;
	eframe::run_native(Box::new(app), native_options);
}