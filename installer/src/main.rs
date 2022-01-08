#![windows_subsystem = "windows"]
use std::{process::{Command, exit}, fs::{File, self}, io::{Write, BufReader}, env};
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
		let args: Vec<String> = env::args().collect();
		if args.contains(&"--update".to_string()) {
			egui::CentralPanel::default().show(ctx, |ui| {
				ui.heading("Update to newses version\nTo Update:");
				ui.label("	- ip2geotools 								  	[ To localization the IP ]");
				ui.label("	- scapy													[ Analyze the Network Traffic ]");
				
				ui.label("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
				
				ui.horizontal(|ui| {			
				
					ui.add_sized((250.,0.), Label::new(" "));

					let window = Window::new("")
							.vscroll(false)
							.collapsible(false)
							.title_bar(false)
							.resizable(false)
							.default_pos((175.,175.));
	
					let text = if self.cont_btn_bool {
						"Working"
					} else {
						"Update"
					};
	
					ui.group(|ui| {
						if ui.button( "Start listener").clicked() {
							Command::new("gta_stuff.exe").spawn().expect("Error Opening listener");
							exit(0);
						}
	
						let continue_btn = ui.button(text);
	
						if continue_btn.clicked() {
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
		} else{
			//BOOK_MARK: FIRST INSTALL
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
	
						let continue_btn = ui.button(text);
	
						if continue_btn.clicked() {
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
	w_version_file().expect("can't create version files");
	Ok(())
}

fn clean_files() -> std::io::Result<()>{
	fs::remove_file("requirements.txt")?;
	fs::remove_file("package.zip")?;
    Ok(())
}

fn w_action(path:&str, content:&str) -> std::io::Result<()>{
	let mut file = std::fs::File::create(path)?;
	std::io::Write::write_all(&mut file, content.as_bytes())?;
	Ok(())
}

fn w_version_file() -> reqwest::Result<()> {
    let body = reqwest::blocking::get("https://raw.githubusercontent.com/Cron3x/GTA_Stuff/main/README.md")?.text()?;
    let sp: Vec<&str> = body.split("\n").collect();
    for i in 0..sp.len() {
        if sp[i].contains("[![version]"){
            let a:Vec<&str> = sp[i].split("version-").collect();
            let b:Vec<&str> = a[1].split("-gree.svg)").collect();
            let c = b[0];
            println!("{}",c);
			w_action(".version", c).expect("could not write version file");
        } 
    }
    Ok(())
}

// 
fn main() {
	let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

	let app = MyEguiApp::default();
	let mut native_options = eframe::NativeOptions::default();
	native_options.initial_window_size = Some(Vec2::new(444., 444.));
	native_options.always_on_top = true;
	eframe::run_native(Box::new(app), native_options);
}