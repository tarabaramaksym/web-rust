use std::collections::HashMap;
use crate::http::{HttpRequest, HttpResponse};
use crate::controller::Controller;

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

		// TODO: add media handling
		if path.ends_with(".ico") {
			response.set_status_code(404);
			return;
		}

        if let Some(controller) = self.routes.get(path) {
            controller.handle_request(request, response);
        } else if let Some(controller) = self.routes.get("/404") {
            controller.handle_request(request, response);
        } else {
            // Set a default 404 response if no 404 route is registered
            response.set_body("404 Not Found".to_string());
        }
    }
}