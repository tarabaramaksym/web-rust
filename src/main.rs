mod http;
mod router;
mod www;
mod controller;
mod template;
mod component;
mod generator;

use std::{net::{TcpListener, TcpStream}, env};
use std::thread;
use crate::http::HttpInitializer;
use crate::generator::StyleGenerator;
use crate::router::Router;
use crate::www::controller::index::HomeController;
use crate::www::controller::about::AboutController;
use crate::www::controller::not_found::NotFoundController;
use std::time::Instant;

fn handle_request(stream: TcpStream, router: &Router) {
	let start = Instant::now();
	
	println!("Received request...");

	let (request, mut response) = HttpInitializer::initialize(stream);
	
	router.execute(&request, &mut response);
	
	response.send_response();

	let duration = start.elapsed();
	println!("Request processed in {:?}", duration);
}

fn register_routes(router: &mut Router) {
	router.register("/", Box::new(HomeController));
	router.register("/about", Box::new(AboutController));
	router.register("/404", Box::new(NotFoundController));
}

fn main() -> std::io::Result<()> {
	println!("Starting server...");

	let style_generator = StyleGenerator::new();
	style_generator.generate();

	let is_dev = env::var("FLY_APP_NAME").is_err();
	
	if is_dev {
		let style_generator_clone = StyleGenerator::new();
		thread::spawn(move || {
			if let Err(e) = style_generator_clone.watch() {
				eprintln!("File watcher error: {}", e);
			}
		});
	}

	let mut router = Router::new();
	register_routes(&mut router);

	let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
	let bind_address = format!("0.0.0.0:{}", port);
	
	println!("Binding to {}", bind_address);
	let listener = TcpListener::bind(&bind_address)?;
	
	println!("Server listening on {}", bind_address);

    for stream in listener.incoming() {
        handle_request(stream?, &router);
    }

	return Ok(());
}