#![allow(unused, dead_code)]
pub fn get_pass() -> String {
	data_init();
	let mut cwd = std::env::current_dir().unwrap_or_else(|_| {
		println!("FATAL: Failed to get current working directory.");
		std::process::exit(1);
	});
	cwd = cwd.join("data");

	let key: String = get_key(0_usize);
	let file = std::fs::read_to_string(cwd.join("1"));
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

pub fn set_pass(pass: String) -> Result<(), std::io::Error> {
	data_init();
	let mut cwd = std::env::current_dir().unwrap_or_else(|_| {
		println!("FATAL: Failed to get current working directory.");
		std::process::exit(1);
	});
	cwd = cwd.join("data");

	let key = get_key(/*pass.len()*/32);
	let nonce = crate::randstring::rand_nonce();
	let encrypted: String = crate::crypto::encrypt(&pass, &key, &nonce);
	let to_write: String = format!("{}\n{}", key, nonce);
	std::fs::write(cwd.join("1"), to_write.as_bytes());
	std::fs::write(cwd.join("2"), encrypted.as_bytes())
}

pub fn get_key(len: usize) -> String {
	data_init();
	let mut cwd = std::env::current_dir().unwrap_or_else(|_| {
		println!("FATAL: Failed to get current working directory.");
		std::process::exit(1);
	});
	cwd = cwd.join("data");

	let file = std::fs::read_to_string(cwd.join("1"));
	if let Ok(s) = file {
		s.trim().into()
	} else {
		let newkey: String = crate::randstring::rand_string(len);
		if std::fs::write(cwd.join("1"), newkey.as_bytes()).is_err() {
			println!("ERROR: Failed to write nonce key to file.");
		}
		newkey
	}
}

pub fn data_init() {
	let mut cwd = std::env::current_dir().unwrap_or_else(|_| {
		println!("FATAL: Failed to get current working directory.");
		std::process::exit(1);
	});
	cwd = cwd.join("data");
	let _ = std::fs::create_dir_all(cwd);
}