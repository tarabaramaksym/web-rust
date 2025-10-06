use crate::http::{HttpRequest};
use crate::template::Template;
use std::collections::HashMap;

pub trait Component: Template {
	fn get_template_variables(&self, request: &HttpRequest) -> HashMap<String, String>;

	fn replace_template_variables(&self, template: String, variables: &HashMap<String, String>) -> String {
		let mut updated_template = template;
		
		while let Some(start) = updated_template.find("{{$") {
			if let Some(relative_end) = updated_template[start..].find("}}") {
				let end = start + relative_end + 2;
				
				let placeholder = updated_template[start..end].to_string();
				let var_name = &placeholder[3..placeholder.len()-2];
				
				if let Some(value) = variables.get(var_name) {
					updated_template = updated_template.replace(&placeholder, value);
				} else {
					break;
				}
			} else {
				break;
			}
		}
		
		updated_template
	}

	fn build_component(&self, request: &HttpRequest) -> String {
		let template = self.render();
		let variables = self.get_template_variables(request);
		
		let final_template = self.replace_template_variables(template, &variables);
		
		return final_template;
	}
}