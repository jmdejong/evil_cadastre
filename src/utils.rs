

pub fn partition(s: &str) -> (String, String) {
	let mut parts: Vec<String> = s.splitn(2, ' ').into_iter().map(String::from).collect();
	while parts.len() < 2 {
		parts.push("".to_string())
	}
	(parts.remove(0), parts.remove(0))
}

pub fn partition_by(s: &str, pat: &str) -> (String, String) {
	let mut parts: Vec<String> = s.splitn(2, pat).into_iter().map(String::from).collect();
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

#[allow(dead_code)]
pub fn ident<T>(t: T) -> T {
	t
}
