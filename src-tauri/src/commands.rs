use crate::vault;

#[tauri::command]
pub async fn login(username: String, password: String) -> Result<(), String> {
	let correct_pass: String = crate::fs::get_pass();
	// println!("Attemped login with {username} and {password}.");
	if password == correct_pass {
		Ok(())
	} else {
		Err("Wrong username or password.".to_string())
	}
}

#[tauri::command]
pub async fn register(username: String, password: String) -> Result<(), String> {
	let res = crate::fs::set_pass(password);
	println!("{res:?}");
	if res.is_ok() {
		Ok(())
	} else {
		Err("Failed to write password to disk.".into())
	}
}

#[tauri::command]
pub fn edit_btn_test() {
	println!("Test");
}

#[tauri::command]
pub async fn save_keys() -> Result<(), String> {
	let mut cwd = std::env::current_dir().unwrap_or_else(|_| {
		println!("FATAL: Failed to get current working directory.");
		std::process::exit(1);
	});
	cwd = cwd.join("data");

	let mut vault = vault::get_vault();
	vault.write_keyfile(cwd.join("3").to_str().unwrap());
	Ok(())
}