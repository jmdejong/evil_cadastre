

pub fn partition(s: &str) -> (String, String) {
	let mut parts: Vec<String> = s.splitn(2, ' ').map(String::from).collect();
	while parts.len() < 2 {
		parts.push("".to_string())
	}
	(parts.remove(0), parts.remove(0))
}

pub fn partition_by(s: &str, pat: &str) -> (String, String) {
	let mut parts: Vec<String> = s.splitn(2, pat).map(String::from).collect();
	while parts.len() < 2 {
		parts.push("".to_string())
	}
	(parts.remove(0), parts.remove(0))
}

pub fn truncated<T: Clone>(a: &[T], l: usize) -> Vec<T> {
	let mut c = a.to_vec();
	if c.len() > l {
		c.resize_with(l, ||{panic!("increasing array during truncation")});
	}
	c
}

pub fn randomize (mut seed: u32) -> u32 {
	seed += 92857;
	seed ^= seed << 13;
	seed ^= seed >> 17;
	seed ^= seed << 5;
	seed
}


#[allow(dead_code)]
pub fn identity<T>(t: T) -> T {
	t
}
