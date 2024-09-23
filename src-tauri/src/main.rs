// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cipher;

use base64::prelude::*;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn encrypt(plaintext: &str, key: &str) -> Result<Vec<u8>, String> {
    let bytes = plaintext.as_bytes();
    cipher::encrypt(bytes, key)
}

#[tauri::command]
fn decrypt(ciphertext: &str, key: &str) -> Result<Vec<u8>, String> {
    let bytes = BASE64_STANDARD.decode(ciphertext).map_err(|e| e.to_string())?;
    println!("{:?}", bytes);
    cipher::decrypt(&bytes, key)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, encrypt, decrypt])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
