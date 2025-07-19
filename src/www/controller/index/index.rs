use std::collections::HashMap;
use crate::http::{HttpResponse, HttpRequest};
use crate::controller::Controller;
use crate::template::Template;
use crate::component::Component;
pub struct HomeController;

impl Controller for HomeController {
	fn get_template_variables(&self, request: &HttpRequest) -> HashMap<String, String> {
		let mut variables = HashMap::new();

		variables.insert("title".to_string(), "Home".to_string());
		variables.insert("name".to_string(), "John Doe".to_string());

		return variables;
	}

    fn execute(&self, request: &HttpRequest, response: &mut HttpResponse) {
    }
}

impl Template for HomeController {
	fn get_template_path(&self) -> String {
		"page/index/index.html".to_string()
	}

	fn get_template_components(&self) -> HashMap<String, Box<dyn Component>> {
		HashMap::new()
	}
}