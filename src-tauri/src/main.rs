#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
mod commands;
mod fs;

fn main() {
	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![commands::login, commands::print_hello])
    	.run(tauri::generate_context!())
    	.expect("error while running tauri application");
}
