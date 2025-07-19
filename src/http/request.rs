use std::{collections::HashMap};
pub struct HttpRequest {
	pub method: String,
	pub path: String,
	pub headers: HashMap<String, String>,
	pub body: String
}

impl HttpRequest {
	pub fn new(method: String, path: String, headers: HashMap<String, String>, body: String) -> Self {
		Self {method, path, headers, body}
	}

	pub fn get_path(&self) -> &str {
		&self.path
	}
}