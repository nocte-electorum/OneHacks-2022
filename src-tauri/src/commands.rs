use crate::vault;

#[tauri::command]
pub async fn login(username: String, password: String) -> Result<(), String> {
	let correct_pass: String = crate::fs::get_pass();
	println!("Attemped login with {username} and {password}.");
	Err("Wrong username or password.".to_string())
	// Ok(())
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
pub async fn save_keys() -> Result<(), String> {
	// Call vault save logic here
	let mut vault = vault::get_vault();
	vault.write_keyfile("./data/3");
	Ok(())
}