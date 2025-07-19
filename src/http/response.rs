use std::{collections::HashMap, io::Write, net::TcpStream};

pub struct HttpResponse {
	stream: TcpStream,
	status_code: u16,
	headers: HashMap<String, String>,
	pub body: String
}

impl HttpResponse {
	pub fn new(mut stream: TcpStream) -> Self {
		Self {stream, status_code: 200, headers: HashMap::new(), body: String::new()}
	}

	pub fn send_response(&mut self) {
		let response_data = format!("HTTP/1.1 {} {}\r\n", self.status_code, self.status_code);
		let response_data = format!("{}Content-Length: {}\r\n\r\n{}", response_data, self.body.len(), self.body);
		let response_bytes = response_data.as_bytes();

		if let Err(e) = self.stream.write_all(response_bytes) {
			println!("Error sending response: {}", e);
		}
	}

	pub fn set_body(&mut self, new_body: String) {
		self.body = new_body;	
	}

	pub fn set_status_code(&mut self, new_status_code: u16) {
		self.status_code = new_status_code;
	}
}