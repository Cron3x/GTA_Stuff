use std::{collections::HashMap, time::SystemTime};
use rand::Rng;
use rusqlite::{Connection, Result};
use chrono::Local;

#[derive(Debug)]
struct GTAConnection {
    ip: String,
    location: String,
    time: String,
}

fn format_connection(input:String) -> HashMap<String, String> {
    let a: Vec<&str> = input.split("{").collect();
    let b: Vec<&str> = a[1].split("}").collect();
    let c = b[0].replace(" ", "");
    let mut d: HashMap<String, String> = HashMap::new();
    let e: Vec<&str> = c.split(",").collect();

    for f in e {
        let g: Vec<&str> = f.split(":").collect();
        //println!("{:?}", g);
        d.insert(g[0].to_string(), g[1].to_string());
    }

    return d;
}

pub fn read(tabel: &str) -> Result<HashMap<usize, HashMap<String, String>>> {

    let mut connection_map = HashMap::new();
    let con = Connection::open("communicate.db")?;
    
    let mut stmt = con.prepare(
        format!("SELECT * FROM {};", tabel).as_str(),
    )?;

    let gta_connections = stmt.query_map([], |row| {
        Ok(GTAConnection {
            time: row.get(0)?,
            ip: row.get(1)?,
            location: row.get(2)?,
        })
    })?;

    let mut i = 0;

    for gta_connection in gta_connections {
        let tmp = format!("{:?}", gta_connection.expect(""));
        connection_map.insert(i, format_connection(tmp));
        i += 1;
    }

    //println!("{:?}", connection_map);

    Ok(connection_map)
}

pub fn create_table() -> Result<(), rusqlite::Error> {
    let con = Connection::open("communicate.db")?;

    let _ = con.execute(format!("CREATE TABLE ips (time text,ip text, location text)").as_str(),[]);  //TODO: Modular machen (Hashmap mit entires)

    let _ = con.close();
    Ok(())
}

pub fn clear(table: &str) -> Result<()> {
    let con = Connection::open("communicate.db")?;

    con.execute(format!("DELETE FROM {}", table).as_str(),[])?;

    con.close().expect("cant close db connection [clear function]");
    Ok(())
}

pub fn fill_with_dummy_data(number:i32) -> Result<(), rusqlite::Error> {
    create_table().unwrap();
    let con = Connection::open("communicate.db")?;
    let mut rng = rand::thread_rng();
    for i in 0..number {
        let ru0: u8 = rng.gen();
        let ru1: u8 = rng.gen();
        let ru2: u8 = rng.gen();
        let ru3: u8 = rng.gen();

        let date = Local::now();

        let _ = con.execute(format!("REPLACE INTO ips VALUES ('{time}','{ip}','{location}')", time=date.format("%H-%M-%S"),ip=format!("{}.{}.{}.{}", ru0,ru1,ru2,ru3), location="Area*51+Nevada+USA").as_str(),[]);
    }
    
    let _ = con.close();
    Ok(())
}