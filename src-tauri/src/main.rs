#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]


#[tauri::command]
fn print_hello() {  // dummy function to test client -> server boundary
	println!("Hello world!");
}


fn main() {
  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
