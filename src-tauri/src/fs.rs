#![allow(unused, dead_code)]
pub fn get_pass() -> String {
	let mut cwd = std::env::current_dir().unwrap_or_else(|_| {
		println!("FATAL: Failed to get current working directory.");
		std::process::exit(1);
	});
	cwd = cwd.join("data");

	let key: String = get_key();
	let file = std::fs::read_to_string(cwd.join("2"));
	if let Ok(s) = file {
		let res = crate::crypto::decrypt(&s, &key, &key);
		if let Ok(s) = res { s } else {
			println!("FATAL: Stored encryption key does not match.");
			std::process::exit(1);
		}
	} else {
		println!("FATAL: Program requested password when user hasn't set one.");
		std::process::exit(1);
	}
}

pub fn get_key() -> String {
	let mut cwd = std::env::current_dir().unwrap_or_else(|_| {
		println!("FATAL: Failed to get current working directory.");
		std::process::exit(1);
	});
	cwd = cwd.join("data");

	let file = std::fs::read_to_string(cwd.join("1"));
	if let Ok(s) = file {
		s.trim().into()
	} else {
		let newkey: String = crate::randstring::rand_string(14);
		if std::fs::write(cwd.join("1"), newkey.as_bytes()).is_err() {
			println!("ERROR: Failed to write nonce key to file.");
		}
		newkey
	}
}