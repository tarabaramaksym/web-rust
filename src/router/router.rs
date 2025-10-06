use std::collections::HashMap;
use crate::http::{HttpRequest, HttpResponse};
use crate::controller::Controller;
use std::fs;

type HandlerController = Box<dyn Controller>;

pub struct Router {
    routes: HashMap<String, HandlerController>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn register(&mut self, path: &str, controller: Box<dyn Controller>) {
        self.routes.insert(path.to_string(), controller);
    }

    pub fn execute(&self, request: &HttpRequest, response: &mut HttpResponse) {
        let path = request.get_path();

		if self.is_media_file(path) {
			self.serve_media_file(path, response);
			return;
		}

        if let Some(controller) = self.routes.get(path) {
            controller.handle_request(request, response);
        } else if let Some(controller) = self.routes.get("/404") {
            controller.handle_request(request, response);
        } else {
            response.set_body("404 Not Found".to_string());
        }
    }

	fn is_media_file(&self, path: &str) -> bool {
        path.ends_with(".ico") || 
        path.ends_with(".png") || 
        path.ends_with(".jpg") || 
        path.ends_with(".jpeg") || 
        path.ends_with(".gif") || 
        path.ends_with(".css") || 
        path.ends_with(".js") || 
        path.ends_with(".svg") ||
        path.ends_with(".woff") ||
        path.ends_with(".woff2") ||
        path.ends_with(".ttf")
    }

    fn serve_media_file(&self, path: &str, response: &mut HttpResponse) {
        let file_path = if path.starts_with('/') {
            &path[1..]
        } else {
            path
        };

        match fs::read(file_path) {
            Ok(file_data) => {
                let content_type = self.get_content_type(path);
                response.set_content_type(&content_type);
                
                self.add_cache_headers(path, response);
                
                if self.is_binary_file(path) {
                    response.set_binary_body(file_data);
                } else {
                    if let Ok(text_content) = String::from_utf8(file_data) {
                        response.set_body(text_content);
                    } else {
                        response.set_status_code(500);
                        response.set_body("Internal Server Error".to_string());
                    }
                }
            }
            Err(_) => {
                response.set_status_code(404);
                response.set_body("File not found".to_string());
            }
        }
    }

    fn get_content_type(&self, path: &str) -> String {
        match path {
            p if p.ends_with(".html") => "text/html".to_string(),
            p if p.ends_with(".css") => "text/css".to_string(),
            p if p.ends_with(".js") => "application/javascript".to_string(),
            p if p.ends_with(".json") => "application/json".to_string(),
            p if p.ends_with(".png") => "image/png".to_string(),
            p if p.ends_with(".jpg") || p.ends_with(".jpeg") => "image/jpeg".to_string(),
            p if p.ends_with(".gif") => "image/gif".to_string(),
            p if p.ends_with(".svg") => "image/svg+xml".to_string(),
            p if p.ends_with(".ico") => "image/x-icon".to_string(),
            p if p.ends_with(".woff") => "font/woff".to_string(),
            p if p.ends_with(".woff2") => "font/woff2".to_string(),
            p if p.ends_with(".ttf") => "font/ttf".to_string(),
            _ => "application/octet-stream".to_string(),
        }
    }

    fn is_binary_file(&self, path: &str) -> bool {
        path.ends_with(".png") || 
        path.ends_with(".jpg") || 
        path.ends_with(".jpeg") || 
        path.ends_with(".gif") || 
        path.ends_with(".ico") ||
        path.ends_with(".woff") ||
        path.ends_with(".woff2") ||
        path.ends_with(".ttf")
    }

    fn add_cache_headers(&self, path: &str, response: &mut HttpResponse) {
        if path.ends_with(".ttf") || path.ends_with(".woff") || path.ends_with(".woff2") {
            response.add_header("Cache-Control", "public, max-age=31536000, immutable");
            response.add_header("ETag", "\"font-v1\"");
        } else if path.ends_with(".png") || path.ends_with(".ico") || path.ends_with(".svg") || 
                  path.ends_with(".jpg") || path.ends_with(".jpeg") || path.ends_with(".gif") {
            response.add_header("Cache-Control", "public, max-age=2592000");
            response.add_header("ETag", "\"img-v1\"");
        } else if path.ends_with(".css") {
            response.add_header("Cache-Control", "public, max-age=3600");
            response.add_header("ETag", "\"css-v1\"");
        } else if path.ends_with(".js") {
            response.add_header("Cache-Control", "public, max-age=3600");
            response.add_header("ETag", "\"js-v1\"");
        } else {
            response.add_header("Cache-Control", "public, max-age=86400");
        }
    }
}