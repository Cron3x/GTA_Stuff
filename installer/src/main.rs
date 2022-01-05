use std::{process::{Command, exit}, fs::File, io::{Write, Seek, Read}, thread};
use eframe::{epi, egui::{self, Vec2, Label, Button, Window}};
use zip::{result::ZipResult, read::ZipFile, ZipArchive};

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


fn browse_zip_archive<T, F, U>(buf: &mut T, browse_func: F) -> ZipResult<Vec<U>>
    where T: Read + Seek,
          F: Fn(&ZipFile) -> ZipResult<U>
{
    let mut archive = ZipArchive::new(buf)?;
    (0..archive.len())
        .map(|i| archive.by_index(i).and_then(|file| browse_func(&file)))
        .collect()
}

fn download_content() -> std::io::Result<()>{
	install_python_dep().expect("can't install python dependencies");

	let mut aw = Command::new("powershell").arg("-Command").arg("(New-Object Net.WebClient).DownloadFile('https://www.dropbox.com/s/lxyp1104buf0iaj/gta_stuff.zip?dl=1', 'package.zip')").spawn()?;
	aw.wait()?;
	//TODO: unzip package.zip
	let mut file = File::open("package.zip").expect("Couldn't open file");
	let files = browse_zip_archive(&mut file, |f| {
		Ok(format!("{}: {} -> {}", f.name(), f.size(), f.compressed_size()))
	});
	println!("{:?}", files);
	Ok(())
}


fn main() {
	let app = MyEguiApp::default();
	let mut native_options = eframe::NativeOptions::default();
	native_options.initial_window_size = Some(Vec2::new(444., 444.));
	eframe::run_native(Box::new(app), native_options);
}