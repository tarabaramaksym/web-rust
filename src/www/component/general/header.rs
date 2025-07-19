use crate::component::Component;
use crate::http::HttpRequest;
use crate::template::Template;
use std::collections::HashMap;

pub struct HeaderComponent;

impl Component for HeaderComponent {
	fn get_template_variables(&self, request: &HttpRequest) -> HashMap<String, String> {
		HashMap::new()
	}
}

impl Template for HeaderComponent {
	fn get_template_path(&self) -> String {
		"component/general/header.html".to_string()
	}

	fn get_template_components(&self) -> HashMap<String, Box<dyn Component>> {
		HashMap::new()
	}
}