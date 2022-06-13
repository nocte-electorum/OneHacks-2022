#![feature(once_cell, let_chains)]
#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
mod commands;
mod fs;
mod crypto;
mod randstring;
mod vault;



fn main() {
	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![
			commands::login, 
			commands::log, 
			commands::save_keys, 
			commands::register, 
			commands::add_pass,
			commands::is_registered,
			commands::get_passwords
		])
    	.run(tauri::generate_context!())  // IDEs will show an error but it should be ignored
    	.expect("error while running tauri application");
}
