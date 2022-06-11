#[tauri::command]
pub async fn login(username: String, password: String) -> Result<(), String> {
	println!("Attemped login with {username} and {password}.");
	Err("Wrong username or password.".to_string())
	// Ok(())
}