// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[tauri::command]
fn save(data: &str){
    // this creates a file, or over-writes it if not found
    let mut file = File::create("save.txt").unwrap();
    file.write_all(data.as_bytes()).unwrap();
}

#[tauri::command]
fn load(name: &str) -> String {
    let file = File::open(name).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    contents
}

#[tauri::command]
fn clear_save(name: &str){
    let mut file = File::create(name).unwrap();
    file.write_all(b"").unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![save, load, clear_save])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
