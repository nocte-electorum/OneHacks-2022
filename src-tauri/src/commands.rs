#[tauri::command]
pub fn print_hello() {  // dummy function to test client -> server boundary
	println!("Hello world!");
}

#[tauri::command]
pub async fn login(username: String, password: String) -> Result<(), String> {
	println!("Attemped login with {username} and {password}.");
	Ok(())
}