use std::collections::HashMap;
use crate::http::{HttpResponse, HttpRequest};
use crate::controller::Controller;
use crate::template::Template;
use crate::component::Component;

pub struct NotFoundController;

impl Controller for NotFoundController {
	fn get_template_variables(&self, request: &HttpRequest) -> HashMap<String, String> {
		let mut variables = HashMap::new();
		variables.insert("title".to_string(), "404".to_string());

		return variables;
	}

    fn execute(&self, request: &HttpRequest, response: &mut HttpResponse) {
		response.set_status_code(404);
    }
}

impl Template for NotFoundController {
	fn get_template_path(&self) -> String {
		"page/404/404.html".to_string()
	}

	
	fn get_template_components(&self) -> HashMap<String, Box<dyn Component>> {
		HashMap::new()
	}
}