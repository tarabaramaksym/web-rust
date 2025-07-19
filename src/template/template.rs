use std::collections::HashMap;
use crate::component::Component;

pub trait Template {
	fn get_template_path(&self) -> String;

	fn get_template_components(&self) -> HashMap<String, Box<dyn Component>>;

	fn render(&self) -> String {
		let template_path = "src/www/template/".to_string() + &self.get_template_path();
		
		return self.render_content(&template_path);
	}

	fn render_content(&self, path: &str) -> String {
		let file_path = path;

		if let Ok(content) = std::fs::read_to_string(&file_path) {
			return content;
		} else {
			return "Template not found".to_string();
		}
	}
}