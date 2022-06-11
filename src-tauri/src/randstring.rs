const CHARSET: [char; 43] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 
'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
'!', '@', '#', '$', '%', '^', '&', '*', '-', '_', '=', '+', '<', '>', '?', '.', ','];


pub fn rand_string(len: usize) -> String {
	let mut rng = rand::thread_rng();
	let mut s = String::new();
	for _ in 0..len {
		s.push(CHARSET[rand::Rng::gen_range(&mut rng, 0..CHARSET.len())]);
	}
	s
}