use aes_gcm::{ Aes256Gcm, Key, Nonce };
use aes_gcm::aead::{ Aead, NewAead };

pub fn encrypt(s: &str, key: &str, nonce: &str) -> String {
	let key = Key::from_slice(key.as_bytes());
	let cipher = Aes256Gcm::new(key);
	let nonce = Nonce::from_slice(nonce.as_bytes());

	let enc = cipher.encrypt(nonce, s.as_bytes())
		.expect("FATAL: Error in AES-256 encryption.");
	String::from_utf8_lossy(&enc).to_string()
}

pub fn decrypt(s: &str, key: &str, nonce: &str) -> Result<String, ()> {
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