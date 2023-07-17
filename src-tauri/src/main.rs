// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{env, fs};
use std::io::{self, Write, Read};
use std::net::{TcpStream, Shutdown, SocketAddr};
use std::thread;
use std::time::{Duration, SystemTime, Instant};
use xmltree::Element;
use xmltree::ParseError;
use thiserror::Error;
use serde::{Deserialize, Serialize};
use tauri::Context;


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

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    ip4: String,
    port: String,
}


fn main() {
    let config_path = tauri::api::path::config_dir().unwrap().join("peachy-pager/config.json");

    if !config_path.exists() {
        // Create a default configuration
        let default_config = r#"{
            "ip": "127.0.0.1"
            "port": "3700"
        }"#;

        fs::create_dir_all(&config_path.parent().unwrap())
            .expect("Failed to create the configuration directory");

        // Write the default configuration to the file
        fs::write(&config_path, default_config)
            .expect("Failed to create the configuration file");
        }

    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![first_time_file, connect, change_config])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
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
    println!("Starting lrsn_listener");
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
    println!("Exiting lrsn_listener");
    Ok("".to_string())
}


#[tauri::command]
fn connect(page_num: &str) -> Result<String, CustomError> {
    let config_path = tauri::api::path::config_dir().unwrap().join("peachy-pager/config.json");
    let config_file = fs::read_to_string(config_path).expect("Failed to read file");
    let config: Config = serde_json::from_str(&config_file).expect("Failed to deserialize JSON");

    println!("Config: {:?}", config);
    let max_retries = 3;
    let timeout_duration = Duration::from_secs(10);

    let mut retry_count = 0;
    loop {
        let start_time = Instant::now();
        let result = TcpStream::connect_timeout(
            &SocketAddr::new(config.ip4.parse().unwrap(), config.port.parse().unwrap()),
            timeout_duration,
        );
        println!("Result: {:?}", result);
        println!("retry_count: {:?}", retry_count);

        match result {
            Ok(stream) => {
                let mut sock = stream;
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
                break; // Successful connection, exit the loop
            }
            Err(err) => {
                retry_count += 1;
                if retry_count <= max_retries {
                    let elapsed_time = Instant::now().duration_since(start_time);
                    if elapsed_time < timeout_duration {
                        let remaining_time = timeout_duration - elapsed_time;
                        println!("Failed to connect, retrying ({}/{}): {}. Retrying after {:?}", retry_count, max_retries, err, remaining_time);
                        thread::sleep(remaining_time);
                    }
                } else {
                    return Err(CustomError::Io(io::Error::new(io::ErrorKind::Other, format!("Failed to connect after {} retries", max_retries))));
                }
            }
        }
    }

    Ok("".to_string())
}

#[tauri::command]
fn change_config(config_as_json_string: &str)-> String{
    let config_path = tauri::api::path::config_dir().unwrap().join("peachy-pager/config.json");
    eprintln!("Result: {:?}", config_as_json_string);
    let result: Result<Config, serde_json::Error> = serde_json::from_str(config_as_json_string);
    eprintln!("Result: {:?}", result.as_ref());
    
     match result {
        Ok(data) => {
            // Serialize the deserialized data back into JSON
            let serialized_data = serde_json::to_string_pretty(&data).expect("Failed to serialize data");

            // Save the JSON data to a file
            std::fs::write(config_path, serialized_data).expect("Failed to write file");
            return format!("Config file updated")
        }

        Err(err) => {
            eprintln!("Failed to deserialize JSON: {}", err);

            return format!("Error: {:?}", err)
        }
    }        

}