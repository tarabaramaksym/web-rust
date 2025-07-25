use crate::http::{HttpResponse, HttpRequest};
use crate::template::Template;
use std::collections::HashMap;
use crate::component::Component;
use crate::www::component::general::{HeaderComponent, FooterComponent};

pub trait Controller: Template {
	fn execute(&self, request: &HttpRequest, response: &mut HttpResponse);

	fn get_template_variables(&self, request: &HttpRequest) -> HashMap<String, String>;

	fn get_general_components(&self) -> HashMap<String, Box<dyn Component>> {
		let mut components: HashMap<String, Box<dyn Component>> = HashMap::new();

		components.insert("header".to_string(), Box::new(HeaderComponent));
		components.insert("footer".to_string(), Box::new(FooterComponent));

		return components;
	}

	fn handle_request(&self, request: &HttpRequest, response: &mut HttpResponse) {
		self.execute(request, response);
		self.after_execute(request, response);
	}

	fn after_execute(&self, request: &HttpRequest, response: &mut HttpResponse) {
		self.build_template(request, response);
	}

	fn replace_template_variables(&self, template: String, variables: &HashMap<String, String>) -> String {
		let mut updated_template = template;
		
		while let Some(start) = updated_template.find("{{$") {
			if let Some(relative_end) = updated_template[start..].find("}}") {
				let end = start + relative_end + 2; // +2 for the "}}"
				
				let placeholder = updated_template[start..end].to_string(); // Convert to owned String
				let var_name = &placeholder[3..placeholder.len()-2]; // Remove "{{$" and "}}"
				
				if let Some(value) = variables.get(var_name) {
					updated_template = updated_template.replace(&placeholder, value);
					println!("Replaced '{}' with '{}'", placeholder, value);
				} else {
					println!("Variable '{}' not found in variables", var_name);
					break;
				}
			} else {
				break;
			}
		}
		
		updated_template
	}

	fn replace_template_components(&self, template: String, request: &HttpRequest) -> String {
		let mut updated_template = template;
		
		// Get all available components
		let mut all_components = self.get_general_components();
		let template_components = self.get_template_components();
		
		// Merge template-specific components with general ones
		for (key, component) in template_components {
			all_components.insert(key, component);
		}

		println!("Current template: {}", updated_template);
		
		// Find all {{#component}} patterns in the template
		while let Some(start) = updated_template.find("{{#") {
			if let Some(relative_end) = updated_template[start..].find("}}") {
				let end = start + relative_end + 2; // +2 for the "}}"
				
				let placeholder = updated_template[start..end].to_string();
				let component_name = &placeholder[3..placeholder.len()-2]; // Remove "{{#" and "}}"
				
				// Replace if we have this component
				if let Some(component) = all_components.get(component_name) {
					// Render the component
					let component_html = component.build_component(request); // Assuming components have a render method
					updated_template = updated_template.replace(&placeholder, &component_html);
					println!("Replaced component '{}' with rendered HTML", placeholder);
				} else {
					println!("Component '{}' not found", component_name);
					break; // Avoid infinite loop
				}
			} else {
				break; // No closing }} found
			}
		}
		
		updated_template
	}

	fn build_template(&self, request: &HttpRequest, response: &mut HttpResponse) {
		let template = self.render_content("src/www/template/page/index.html");

		if template.is_empty() {
			response.set_status_code(404);
			return;
		}

		println!("Template: {}", template);

		let mut variables = self.get_template_variables(request);
		
		let page = self.render();
		let page_with_variables = self.replace_template_variables(page, &variables);

		println!("Page with variables: {}", page_with_variables);
		
		variables.insert("page".to_string(), page_with_variables);

		// First replace variables
		let template_with_variables = self.replace_template_variables(template, &variables);
		
		println!("Template with variables: {}", template_with_variables);
		
		// Then replace components
		let final_template = self.replace_template_components(template_with_variables, request);
		
		response.set_body(final_template);
	}
}