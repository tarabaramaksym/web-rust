use crate::component::Component;
use crate::http::HttpRequest;
use crate::template::Template;
use std::collections::HashMap;
use chrono::{Datelike, Utc};

pub struct FooterComponent;

impl Component for FooterComponent {
	fn get_template_variables(&self, request: &HttpRequest) -> HashMap<String, String> {
		let mut variables = HashMap::new();

		let year = Utc::now().year();


		variables.insert("year".to_string(), year.to_string());

		return variables;
	}
}

impl Template for FooterComponent {
	fn get_template_path(&self) -> String {
		"component/general/footer.html".to_string()
	}

	fn get_template_components(&self) -> HashMap<String, Box<dyn Component>> {
		HashMap::new()
	}
}