use std::fs;
use std::path::Path;
use std::time::Duration;
use std::thread;
use std::sync::mpsc;
use notify::{Watcher, RecursiveMode, Result, Event, EventKind};

pub struct StyleGenerator {
}

impl StyleGenerator {
	pub fn new() -> Self {
		StyleGenerator {}
	}

	pub fn generate(&self) {
		if let Err(e) = self.generate_styles() {
			eprintln!("Error generating styles: {}", e);
		}
	}

	pub fn minimize_style(&self, style: String) -> String {
		let minimized_style = style
			.lines()
			.map(|line| line.trim())
			.filter(|line| !line.is_empty() && !line.starts_with("/*") && !line.starts_with("*/"))
			.collect::<Vec<&str>>()
			.join(" ");

		let minimized_style = minimized_style
			.replace("  ", " ")
			.replace("  ", " ") // Double replace to catch multiple spaces
			.replace(" {", "{")
			.replace("{ ", "{")
			.replace(" }", "}")
			.replace("} ", "}")
			.replace("; ", ";")
			.replace(";", "; ")
			.replace(": ", ":")
			.replace(": ", ":") // Double replace for consistency
			.replace(", ", ",")
			.replace("; ", ";")
			.replace("; ", ";") // Double replace for consistency
			.trim()
			.to_string();

		minimized_style
	}

	pub fn watch(&self) -> Result<()> {
		let (tx, rx) = mpsc::channel();
		let mut watcher = notify::recommended_watcher(tx)?;
		
		watcher.watch(Path::new("src/www/style"), RecursiveMode::Recursive)?;
		
		println!("Style watcher started. Monitoring src/www/style for changes...");
		
		loop {
			match rx.recv() {
				Ok(event) => {
					if let Ok(event) = event {
						self.handle_file_event(event);
					}
				}
				Err(e) => {
					eprintln!("Error receiving file event: {}", e);
					break;
				}
			}
		}
		
		Ok(())
	}

	pub fn regenerate(&self) -> Result<()> {
		println!("Regenerating styles...");
		
		match self.generate_styles() {
			Ok(_) => println!("Styles regenerated successfully"),
			Err(e) => {
				eprintln!("Error regenerating styles: {}", e);
				return Err(e);
			}
		}
		
		Ok(())
	}

	fn generate_styles(&self) -> Result<()> {
		let styles = self.collect_css_files("src/www/style")?;
		
		let minimized_styles: Vec<String> = styles
			.into_iter()
			.filter_map(|entry| {
				fs::read_to_string(entry.path()).ok()
			})
			.map(|style| self.minimize_style(style))
			.collect();

		let minimized_styles = minimized_styles.join("\n");

		fs::create_dir_all("pub/generated")?;
		fs::write("pub/generated/style.css", minimized_styles)?;
		
		Ok(())
	}

	fn collect_css_files(&self, dir_path: &str) -> Result<Vec<fs::DirEntry>> {
		let mut css_files = Vec::new();
		let entries = fs::read_dir(dir_path)?;
		
		for entry in entries {
			let entry = entry?;
			let path = entry.path();
			
			if path.is_dir() {
				let mut subdir_files = self.collect_css_files(path.to_str().unwrap())?;
				css_files.append(&mut subdir_files);
			} else if path.extension().map(|ext| ext == "css").unwrap_or(false) {
				css_files.push(entry);
			}
		}
		
		Ok(css_files)
	}

	fn handle_file_event(&self, event: Event) {
		match event.kind {
			EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) => {
				if let Some(path) = event.paths.first() {
					if path.extension().map(|ext| ext == "css").unwrap_or(false) {
						println!("CSS file changed: {:?}", path);
						
						// Debounce: wait a bit before regenerating
						thread::sleep(Duration::from_millis(100));
						
						if let Err(e) = self.regenerate() {
							eprintln!("Failed to regenerate styles: {}", e);
						}
					}
				}
			}
			_ => {}
		}
	}
}