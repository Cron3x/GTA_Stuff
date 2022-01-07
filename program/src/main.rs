#![windows_subsystem = "windows"]

use std::{process::{Command, exit}, thread};

mod gui_udp_sniffer;
mod data_base;


fn send_get_req() -> Result<(), reqwest::Error> {
    let body = reqwest::blocking::get("https://www.rust-lang.org")?.text()?;
    Ok(())
}

fn new_version() -> bool {
    
    let t = thread::spawn(move || {
        send_get_req()
    });

    return false;
}

fn main() {

    if new_version() {
        Command::new("").arg("--update").spawn().expect("");
        exit(0);
    }


    gui_udp_sniffer::main();
}