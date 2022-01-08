//#![windows_subsystem = "windows"]

use std::{fs,process::{Command, exit}, thread};
mod gui_udp_sniffer;
mod data_base;


fn new_version() -> Result<(), reqwest::Error> {
    let body = reqwest::blocking::get("https://raw.githubusercontent.com/Cron3x/GTA_Stuff/main/README.md")?.text()?;
    let sp: Vec<&str> = body.split("\n").collect();
    for i in 0..sp.len() {
        if sp[i].contains("[![version]"){
            let a:Vec<&str> = sp[i].split("version-").collect();
            let b:Vec<&str> = a[1].split("-gree.svg)").collect();
            let c = b[0];
            println!("{}",c);
            
            let version = rf().expect("msg");
            if version == c{

            } 
        } 
    }
    Ok(())
}

fn rf() -> std::io::Result<String>{
    let contents = fs::read_to_string(".version") 
        .expect("Something went wrong reading the file");
    Ok(contents)
}

fn main() {

    new_version().expect("msg");

    // if new_version() {
    //     Command::new("").arg("--update").spawn().expect("");
    //     exit(0);
    // }


    gui_udp_sniffer::main();
}