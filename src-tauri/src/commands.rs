#[tauri::command]
pub async fn login(username: String, password: String) -> Result<(), String> {
	Err("Wrong username or password.".to_string())
	// println!("Attemped login with {username} and {password}.");
	// Ok(())
}