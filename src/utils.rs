

pub fn partition(s: &str) -> (String, String) {
	let mut parts: Vec<String> = s.splitn(2, ' ').into_iter().map(String::from).collect();
	while parts.len() < 2 {
		parts.push("".to_string())
	}
	(parts.remove(0), parts.remove(0))
}
