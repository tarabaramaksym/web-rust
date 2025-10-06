use std::{collections::HashMap, io::Write, net::TcpStream};

pub struct HttpResponse {
    stream: TcpStream,
    status_code: u16,
    headers: HashMap<String, String>,
    pub body: String,
    binary_body: Option<Vec<u8>>,
}

impl HttpResponse {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream, 
            status_code: 200, 
            headers: HashMap::new(), 
            body: String::new(),
            binary_body: None,
        }
    }

    pub fn send_response(&mut self) {
        let response_data = format!("HTTP/1.1 {} {}\r\n", self.status_code, self.status_code);
        
        let mut header_string = String::new();
        for (key, value) in &self.headers {
            header_string.push_str(&format!("{}: {}\r\n", key, value));
        }
        
        let content_length = if let Some(ref binary_data) = self.binary_body {
            binary_data.len()
        } else {
            self.body.len()
        };
        
        let response_data = format!("{}{}Content-Length: {}\r\n\r\n", 
            response_data, header_string, content_length);
        
        let response_bytes = response_data.as_bytes();
        
        if let Err(e) = self.stream.write_all(response_bytes) {
            println!("Error sending response headers: {}", e);
            return;
        }
        
        if let Some(ref binary_data) = self.binary_body {
            if let Err(e) = self.stream.write_all(binary_data) {
                println!("Error sending binary response: {}", e);
            }
        } else {
            if let Err(e) = self.stream.write_all(self.body.as_bytes()) {
                println!("Error sending text response: {}", e);
            }
        }
    }

    pub fn set_body(&mut self, new_body: String) {
        self.body = new_body;
        self.binary_body = None;
    }

    pub fn set_binary_body(&mut self, binary_data: Vec<u8>) {
        self.binary_body = Some(binary_data);
        self.body.clear();
    }

    pub fn set_content_type(&mut self, content_type: &str) {
        self.headers.insert("Content-Type".to_string(), content_type.to_string());
    }

    pub fn add_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    pub fn set_status_code(&mut self, new_status_code: u16) {
        self.status_code = new_status_code;
    }

    pub fn preload_css(&mut self, css_path: &str) {
        self.add_header("Link", &format!("<{}>; rel=preload; as=style", css_path));
    }

    pub fn add_cache_headers(&mut self) {
        self.add_header("Cache-Control", "public, max-age=31536000");
        self.add_header("ETag", "\"css-v1\"");
    }
}