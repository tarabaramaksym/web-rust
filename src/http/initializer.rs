use std::{collections::HashMap, io::{BufRead, BufReader, Read, Write}, net::{TcpStream}};
use crate::http::{HttpResponse, HttpRequest};

pub struct HttpInitializer {
}

impl HttpInitializer {
	pub fn initialize(stream: TcpStream) -> (HttpRequest, HttpResponse) {
		let mut reader = BufReader::new(stream);
	
		let mut headers = HashMap::new();
		let mut body = String::new();
		
		let mut line = String::new();
	
		let mut request_line = String::new();
		reader.read_line(&mut request_line).unwrap();

		let parts: Vec<&str> = request_line.trim().split_whitespace().collect();
		let method = parts[0].to_string();
		let path = parts[1].to_string();
	
		while reader.read_line(&mut line).unwrap() > 0 {
			if line.trim().is_empty() {
				break;
			}
	
			if let Some((key, value)) = line.split_once(':') {
				let key = key.trim().to_string();
				let value = value.trim().to_string();
	
				headers.insert(key, value);
			}
	
			line.clear();
		}
	
		if let Some(value) = headers.get("Content-Length") {
			if let Ok(size) = value.parse::<usize>() {
				let mut buffer = vec![0; size];
	
				if reader.read_exact(&mut buffer).is_ok() {
					body = String::from_utf8_lossy(&buffer).to_string();
				}
			}
		}

		let request = HttpRequest::new(method, path, headers, body);
		
		let mut stream = reader.into_inner();
		let response = HttpResponse::new(stream);

		return (request, response);
	}
}