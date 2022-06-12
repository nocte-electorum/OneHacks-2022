#![allow(unused, dead_code)]
use std::collections::HashMap;
use std::path::PathBuf;

pub struct Vault {
	passwords: HashMap<String, String>
}

// Constructors
impl Vault {
	pub fn from_keyfile<T: ToString>(raw_path: T) -> Result<Self, VaultError> {
		let path: PathBuf = PathBuf::from(raw_path.to_string());
		let file: String = std::fs::read_to_string(&path)?;
		let buffer: HashMap<String, String> = HashMap::new();

		for line in file.lines() {
			let split: Vec<&str> = line.split('|').collect();
			if split.len() != 2 { return Err(VaultError::new("FATAL: Corrupted keyfile!")) }
			buffer.insert(split[0].into(), split[1].into());
		}

		Ok( Self { passwords: buffer } )
	}
}

// Methods
impl Vault {
	pub fn add(&mut self, name: String, password: String) -> Option<String> {
		self.passwords.insert(name, password)
	}

	pub fn get(&self, name: String) -> Option<String> {
		self.get(name)
	}
}



#[derive(Clone, Debug)]
struct VaultError {
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
    fn from(E: std::io::Error) -> Self {
		VaultError::new(format!("{E:?}"))
    }
}