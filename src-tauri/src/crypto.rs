#![allow(unused, dead_code)]
use aes_gcm::{ Aes256Gcm, Key, Nonce };
use aes_gcm::aead::{ Aead, NewAead };

// Symmetric key-based encryption
// Using bitwise xor
pub fn encrypt(s: &str, key_bytes: &[u8]) -> Vec<u8> {
	let bytes: &[u8] = s.as_bytes();
	let mut final_bytes: Vec<u8> = Vec::new();

	for (i, byte) in bytes.iter().enumerate() {
		final_bytes.push(*byte ^ key_bytes[i])
	}
	
	final_bytes
}

pub fn decrypt(final_bytes: &[u8], key_bytes: &[u8]) -> String {
	let mut reversed_bytes: Vec<u8> = Vec::new();
	for (i, b) in final_bytes.iter().enumerate() {
		reversed_bytes.push(b ^ key_bytes[i]);
	}
	String::from_utf8_lossy(&reversed_bytes).to_string()
}

pub fn write_bytes<T: ToString>(raw_path: T, bytes: &[u8]) -> Result<(), std::io::Error> {
	let mut s: String = String::new();

	for (i, b) in bytes.iter().enumerate() {
		s.push_str(&b.to_string());
		if i != bytes.len() - 1 {
			s.push(' ');
		}
	}

	std::fs::write(
		std::path::PathBuf::from(raw_path.to_string()),
		s.as_bytes()
	)?;
	Ok(())
}

pub fn bytes_writable(bytes: &[u8]) -> String {
	let mut s: String = String::new();

	for (i, b) in bytes.iter().enumerate() {
		s.push_str(&b.to_string());
		if i != bytes.len() - 1 {
			s.push(' ');
		}
	}

	s
}

pub fn read_bytes<T: ToString>(raw_path: T) -> Result<Vec<u8>, std::io::Error> {
	let path = std::path::PathBuf::from(raw_path.to_string());
	let raw = std::fs::read_to_string(path)?;

	let sbytes: Vec<&str> = raw.split(' ').collect::<Vec<&str>>();
	let mut bytes: Vec<u8> = Vec::new();
	for sb in sbytes {
		if !sb.trim().is_empty() {
			bytes.push(sb.parse::<u8>().unwrap());
		}
	}
	Ok(bytes)
}


pub fn encrypt_aes(s: &str, key: &str, nonce: &str) -> String {
	let key = Key::from_slice(key.as_bytes());
	let cipher = Aes256Gcm::new(key);
	let nonce = Nonce::from_slice(nonce.as_bytes());

	let enc = cipher.encrypt(nonce, s.as_bytes())
		.expect("FATAL: Error in AES-256 encryption.");
	String::from_utf8_lossy(&enc).to_string()
}

pub fn decrypt_aes(s: &str, key: &str, nonce: &str) -> Result<String, ()> {
	let key = Key::from_slice(key.as_bytes());
	let cipher = Aes256Gcm::new(key);
	let nonce = Nonce::from_slice(nonce.as_bytes());

	let enc = cipher.decrypt(nonce, s.as_bytes());
		// .expect("FATAL: Error in AES-256 encryption.");
	
	if let Ok(bytes) = enc {
		Ok(String::from_utf8_lossy(&bytes).to_string())
	} else {
		Err(())
	}
}