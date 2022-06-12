#![allow(unused, dead_code)]
use std::collections::HashMap;
use std::f32::consts::E;
use std::lazy::SyncLazy;
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard};

#[derive(Debug, Clone)]
pub struct Vault {
	passwords: HashMap<String, String>
}

unsafe impl Sync for Vault {}

// Constructors/Save functions
impl Vault {
	pub fn from_keyfile<T: ToString>(raw_path: T) -> Result<Self, VaultError> {
		let path: PathBuf = PathBuf::from(raw_path.to_string());
		let file: String = std::fs::read_to_string(&path)?;
		let mut buffer: HashMap<String, String> = HashMap::new();

		for line in file.lines() {
			let split: Vec<&str> = line.split('|').collect();
			if split.len() != 2 { return Err(VaultError::new("FATAL: Corrupted keyfile!")) }
			buffer.insert(split[0].into(), split[1].into());
		}

		Ok( Self { passwords: buffer } )
	}

	pub fn write_keyfile<T: ToString>(&mut self, raw_path: T) -> Result<(), VaultError> {
		crate::fs::data_init();
		let path: PathBuf = PathBuf::from(raw_path.to_string());
		println!("{:?}", &path);
		let buffer: String = "".to_string();

		for (name, pass) in self.passwords.iter() {
			let key: String = crate::fs::get_key();
			let name_enc: String = crate::crypto::encrypt(name, &key, &key);
			let pass_enc: String = crate::crypto::encrypt(name, &key, &key);
		}

		if let Err(e) = std::fs::write(path, buffer.as_bytes()) {
			return Err(VaultError::new(format!("{e:?}")))
		}
		Ok(())
	}

	pub fn new() -> Self {
		let mut cwd = std::env::current_dir().unwrap_or_else(|_| {
			println!("FATAL: Failed to get current working directory.");
			std::process::exit(1);
		});
		cwd = cwd.join("data");

		let file = std::fs::read_to_string(cwd.join("3"));
		if let Err(_) = file {
			Self { passwords: HashMap::new() }
		} else {
			// let res = Vault::from_keyfile(file);
			Vault::from_keyfile(file.unwrap()).unwrap_or_else(|_| {
				println!("An unexpected error occurred!  Were the files corrupted?");  // Theoretically unreachable
				std::process::exit(1);
			})
		}
	}
}

// Methods
impl Vault {
	pub fn add(&mut self, name: String, password: String) -> Option<String> {
		self.passwords.insert(name, password)
	}

	pub fn get(&self, name: String) -> Option<&String> {
		self.passwords.get(&name)
	}
}


pub fn get_vault() -> MutexGuard<'static, Vault> {
	static vault: SyncLazy<Mutex<Vault>> = SyncLazy::new(|| Mutex::new(Vault::new()));
	(*vault).lock().unwrap()
}



#[derive(Clone, Debug)]
pub struct VaultError {
	msg: String
}

impl std::fmt::Display for VaultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.msg);
		Ok(())
    }
}

impl std::error::Error for VaultError {}

impl VaultError {
	pub fn new<T: ToString>(m: T) -> Self {
		Self { msg: m.to_string() }
	}
}

impl From<std::io::Error> for VaultError {
    fn from(e: std::io::Error) -> Self {
		VaultError::new(format!("{e:?}"))
    }
}

impl From<Result<(), std::io::Error>> for VaultError {
    fn from(r: Result<(), std::io::Error>) -> Self {
        if let Err(e) = r {
			VaultError::new(format!("{e:?}"))
		} else {
			VaultError::new("")
		}
    }
}