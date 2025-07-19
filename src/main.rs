mod http;
mod router;
mod www;
mod controller;
mod template;
mod component;

use std::{net::{TcpListener, TcpStream}};
use crate::http::HttpInitializer;
use crate::router::Router;
use crate::www::controller::index::HomeController;
use crate::www::controller::about::AboutController;
use crate::www::controller::not_found::NotFoundController;

fn handle_request(stream: TcpStream, router: &Router) {
	println!("Received request...");

	let (request, mut response) = HttpInitializer::initialize(stream);
	
	router.execute(&request, &mut response);
	
	response.send_response();
}

fn register_routes(router: &mut Router) {
	router.register("/", Box::new(HomeController));
	router.register("/about", Box::new(AboutController));
	router.register("/404", Box::new(NotFoundController));
}

fn main() -> std::io::Result<()> {
	println!("Starting server...");

	let mut router = Router::new();
	register_routes(&mut router);

	let listener = TcpListener::bind("127.0.0.1:3001")?;

    for stream in listener.incoming() {
        handle_request(stream?, &router);
    }

	return Ok(());
}