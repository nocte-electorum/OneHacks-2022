#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]


#[tauri::command]
fn print_hello() {  // dummy function to test client -> server boundary
	println!("Hello world!");
}

#[tauri::command]
async fn login(username: String, password: String) -> Result<(), String> {
	println!("Attemped login with {username} and {password}.");
	Ok(())
}


fn main() {
	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![login, print_hello])
    	.run(tauri::generate_context!())
    	.expect("error while running tauri application");
}
