#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
mod commands;
mod fs;
mod crypto;
mod randstring;

fn main() {
	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![commands::login])
    	.run(tauri::generate_context!())  // IDEs will show an error but it should be ignored
    	.expect("error while running tauri application");
}
