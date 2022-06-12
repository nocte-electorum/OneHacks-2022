#![allow(unused_variables)]
use std::collections::HashMap;
use crate::vault;

#[tauri::command]
pub async fn login(username: String, password: String) -> Result<(), String> {
	let correct_pass: String = crate::fs::get_pass();
	// println!("Attempted login with {username} and {password}.");
	if password == correct_pass {
		Ok(())
	} else {
		Err("Wrong username or password.".to_string())
	}
}

#[tauri::command]
pub async fn register(username: String, password: String) -> Result<(), String> {
	let res = crate::fs::set_pass(password);
	if res.is_ok() {
		Ok(())
	} else {
		Err("Failed to write password to disk.".into())
	}
}

#[tauri::command]
pub fn log(s: String) {
	println!("{s}");
}

#[tauri::command]
pub async fn save_keys() -> Result<(), String> {
	let mut cwd = std::env::current_dir().unwrap_or_else(|_| {
		println!("FATAL: Failed to get current working directory.");
		std::process::exit(1);
	});
	cwd = cwd.join("data");

	let mut vault = vault::get_vault();
	let _ = vault.write_keyfile(cwd.join("3").to_str().unwrap());
	Ok(())
}

#[tauri::command]
pub fn add_pass(name: String, username: String, password: String) -> Result<(), String> {
	let mut vault = vault::get_vault();
	vault.add(name, username, password);
	Ok(())
}

#[tauri::command]
pub fn is_registered() -> bool {
	let res = std::fs::read("./data/2");
	if res.is_ok() { true } else { false }
}

#[tauri::command]
pub fn get_passwords() -> HashMap<String, (String, String)> {
	let vault = vault::get_vault();
	vault.passwords.clone()
}