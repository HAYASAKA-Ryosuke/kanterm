#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::{time::Duration, sync::Mutex, io, borrow::BorrowMut};

use tauri::{State};

#[derive(Debug, serde::Serialize)]
enum MyError {
  FooError,
}

type SerialPortResult<T> = Result<T, Box<dyn std::error::Error>>;

struct TtyPortState {
    port: Mutex<Option<Box<dyn serialport::SerialPort>>>,
}

//fn fetch_ports() -> Vec<SerialPortInfo> {
#[tauri::command]
fn fetch_ports() -> Vec<String> {
    let ports = serialport::available_ports().expect("No ports found!");
    let mut result:Vec<String> = Vec::new();
    for item in ports.iter() {
        result.push(format!("{}", item.port_name));
    }
    return result
}

#[tauri::command]
pub(crate) fn close_port(state: State<'_, TtyPortState>) {
    *state.port.lock().unwrap() = Default::default()
}

#[tauri::command]
pub(crate) fn open_port(path: &str, baud_rate: u32, state: State<TtyPortState>) {
    let port= serialport::new(path, baud_rate)
        .stop_bits(serialport::StopBits::One)
        .data_bits(serialport::DataBits::Eight)
        .timeout(Duration::from_millis(10))
        .open()
        .unwrap_or_else(|e| {
            eprintln!("Failed to open \"{}\". Error: {}", path, e);
            ::std::process::exit(1);
    });
    *state.port.lock().unwrap() = Some(port);
}

#[tauri::command]
fn write(data: &str, state: State<'_, TtyPortState>) {
    state.port.lock().as_mut().unwrap().as_mut().unwrap().write(&data.as_bytes());
}

#[tauri::command]
fn readlines(state: State<TtyPortState>) -> String{
    let mut buffer = String::new();
    let mut buf = [0; 1000];
    return match state.port.lock().as_mut().unwrap().as_mut().unwrap().read(buf.as_mut_slice()) {
        Ok(t) => std::str::from_utf8(&buf[..t]).unwrap().to_string(),
        Err(_e) => format!(""),
        _ => format!("")
    }
}

/*
use std::{collections::HashMap, sync::Mutex};
use tauri::State;
// here we use Mutex to achieve interior mutability
struct Storage {
  store: Mutex<HashMap<u64, String>>,
}
struct Connection;
struct DbConnection {
  db: Mutex<Option<Connection>>,
}

#[tauri::command]
fn connect(connection: State<DbConnection>) {
  // initialize the connection, mutating the state with interior mutability
  *connection.db.lock().unwrap() = Some(Connection {});
}

#[tauri::command]
fn storage_insert(key: u64, value: String, storage: State<Storage>) {
  // mutate the storage behind the Mutex
  storage.store.lock().unwrap().insert(key, value);
}

tauri::Builder::default()
  .manage(Storage { store: Default::default() })
  .manage(DbConnection { db: Default::default() })
  .invoke_handler(tauri::generate_handler![connect, storage_insert])
  // on an actual app, remove the string argument
  .run(tauri::generate_context!("test/fixture/src-tauri/tauri.conf.json"))
  .expect("error while running tauri application");
 */

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch_ports, write, close_port, open_port, readlines])
        .manage(TtyPortState{ port: Default::default() })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
