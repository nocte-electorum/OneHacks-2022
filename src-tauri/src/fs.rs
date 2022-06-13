#![allow(unused, dead_code)]
pub fn get_pass() -> String {
	data_init();
	let mut cwd = std::env::current_dir().unwrap_or_else(|_| {
		println!("FATAL: Failed to get current working directory.");
		std::process::exit(1);
	});
	cwd = cwd.join("data");

	let (key, nonce) = get_key(0_usize);
	println!("Retrieved key {} and nonce {}", &key, &nonce);
	let file = crate::crypto::read_bytes(cwd.join("2").to_str().unwrap());
	if let Ok(s) = file {
		let res = crate::crypto::decrypt(&s, key.as_bytes());
		res
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

	let (key, nonce) = get_key(pass.len());
	let nonce = crate::randstring::rand_nonce();
	let encrypted_b8: Vec<u8> = crate::crypto::encrypt(&pass, key.as_bytes());
	let encrypted: String = String::from_utf8_lossy(&encrypted_b8).to_string();
	let to_write: String = format!("{}\n{}", key.trim(), nonce.trim());
	std::fs::write(cwd.join("1"), to_write.as_bytes());
	println!("Writing key {} and nonce {}", &key, &nonce);
	// std::fs::write(cwd.join("2"), encrypted.as_bytes())
	crate::crypto::write_bytes(cwd.join("2").to_str().unwrap(), &encrypted_b8)
}

pub fn get_key(len: usize) -> (String, String) {
	data_init();
	let mut cwd = std::env::current_dir().unwrap_or_else(|_| {
		println!("FATAL: Failed to get current working directory.");
		std::process::exit(1);
	});
	cwd = cwd.join("data");

	let file = std::fs::read_to_string(cwd.join("1"));
	if let Ok(s) = file {
		let lines = s.lines().collect::<Vec<&str>>();
		let (key, nonce) = (lines[0].trim().into(), lines[1].trim().into());
		return(key, nonce)
	} else {
		(crate::randstring::rand_string(32), crate::randstring::rand_nonce())
	}
}

pub fn get_key_dyn(len: usize) -> (String, String) {
	data_init();
	let mut cwd = std::env::current_dir().unwrap_or_else(|_| {
		println!("FATAL: Failed to get current working directory.");
		std::process::exit(1);
	});
	cwd = cwd.join("data");

	let file = std::fs::read_to_string(cwd.join("1"));
	if let Ok(s) = file && s.len() > len {
		let lines = s.lines().collect::<Vec<&str>>();
		let (key, nonce) = (lines[0].trim().into(), lines[1].trim().into());
		return(key, nonce)
	} else {
		let newkey: String = crate::randstring::rand_string(len);
		let newnonce: String = crate::randstring::rand_nonce();
		let to_write: String = format!("{}\n{}", newkey, newnonce);
		if std::fs::write(cwd.join("1"), to_write.as_bytes()).is_err() {
			println!("ERROR: Failed to write nonce key to file.");
		}
		(newkey, newnonce)
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