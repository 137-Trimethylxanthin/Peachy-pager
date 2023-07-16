// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::io::{self, Write, Read};
use std::net::{TcpStream, Shutdown};
use std::thread;
use std::time::{Duration, SystemTime};
use xmltree::Element;
use xmltree::ParseError;
use thiserror::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Error)]
enum CustomError {
    #[error(transparent)]
    Io(#[from] io::Error),
}

impl serde::Serialize for CustomError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}


fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![sendNewPage, first_time_file, connect])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn sendNewPage(page_num:&str) -> String {
  let parsed_number: Result<i32, _> = page_num.parse();

    match parsed_number {
        Ok(number) => {
            return format!("Pager Nummer: {} wurde gepingt", number);
        }
        Err(e) => {
            return format!("Failed to parse number: {}", e);
        }
    }
}


#[tauri::command]
fn first_time_file() -> String {
//config file
  let config_path = tauri::api::path::config_dir();
  format!("Path: {:?}", config_path.unwrap())
}


fn read(sock: &mut TcpStream, timeout: u64) -> io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    let start = SystemTime::now();

    loop {
        let mut chunk = vec![0u8; 1024];
        match sock.read(&mut chunk) {
            Ok(bytes_read) => {
                buf.extend(&chunk[..bytes_read]);
                if chunk.is_empty() || buf.contains(&b'\n') {
                    return Ok(buf);
                }
            },
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock || e.kind() == io::ErrorKind::TimedOut => {
                if start.elapsed().unwrap() >= Duration::from_secs(timeout) {
                    return Ok(buf);
                }
                thread::sleep(Duration::from_millis(100));
            },
            Err(e) => return Err(e),
        }
    }
}


fn lrsn_listener(sock: &mut TcpStream) -> Result<String, CustomError>  {
    let mut first_heartbeat_received = false;
    let mut last_heartbeat = SystemTime::now();
    let mut heartbeat_interval = Duration::from_secs(0);

    loop {
        let data = read(sock, 5)?;

        if data.is_empty() {
            let elapsed = last_heartbeat.elapsed().unwrap();
            if first_heartbeat_received && elapsed >= heartbeat_interval {
                println!("Did not receive LRSN Heartbeat within {:?} since {:?}. Closing.", heartbeat_interval, last_heartbeat);
                break;
            }
        } else {
            let root = Element::parse(data.as_slice()).expect("some error");
            println!("Got: {:?}", root);
            
            if root.name == "Heartbeat" {
                if let Some(interval_str) = root.attributes.get("interval") {
                    heartbeat_interval = Duration::from_secs(interval_str.parse::<u64>().unwrap());
                }
                last_heartbeat = SystemTime::now();
                first_heartbeat_received = true;
            }
        }
    }

    Ok("".to_string())
}


#[tauri::command]
fn connect(ip_address: String, page_num: &str) -> Result<String, CustomError> {
    let mut sock = TcpStream::connect(format!("{}:3700", ip_address))?;
    sock.set_nodelay(true)?;

    println!("Got: {:?}", read(&mut sock, 5));

    let login_msg = "<Login services=\"NetPage;Heartbeat\" />\n";
    sock.write_all(login_msg.as_bytes())?;

    let mut buffer = Vec::new();
    sock.read_to_end(&mut buffer)?;
    let root = Element::parse(buffer.as_slice()).map_err(|e| {
        CustomError::Io(io::Error::new(io::ErrorKind::Other, format!("XML parse error: {}", e)))
    })?;

    if root.name != "LoginAck" || root.attributes.get("ret") != Some(&"0".to_string()) {
        println!("Unable to log in. Received: {:?}", root.attributes.get("ret"));
        return Err(CustomError::Io(io::Error::new(io::ErrorKind::Other, "Unable to log in")));
    }

    sock.set_nonblocking(true)?;

    // Start the lrsn_listener in a separate thread
    let mut second_sock = sock.try_clone()?;
    thread::spawn(move || {
        if let Err(e) = lrsn_listener(&mut second_sock) {
            println!("Error in lrsn_listener: {:?}", e);
        }
    });

    let number = match page_num.parse::<i32>() {
        Ok(number) => number,
        Err(e) => {
            println!("Failed to parse number: {}", e);
            return Err(CustomError::Io(io::Error::new(io::ErrorKind::Other, "Failed to parse number")));
        }
    };

    let pager_type = 0; // Assuming a default pager type of 0
    let message = "Vibe1".to_string(); // Assuming a default message of "Vibe1"

    let page_request = format!(
        "<PageRequest id=\"{}\" pager=\"{};{}\" message=\"{}\" />\n",
        number, pager_type, number, message
    );
    sock.write_all(page_request.as_bytes())?;

    sock.shutdown(Shutdown::Both)?;
    Ok("".to_string())
}