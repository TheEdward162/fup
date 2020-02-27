use std::io::{BufRead, Write};

pub const REPL_PROMPT: &str = "( ";

fn repl_prompt(stderr: &std::io::Stderr) {
	let mut lock = stderr.lock();
	lock.write(REPL_PROMPT.as_bytes())
		.expect("Could not write to stderr");
	lock.flush().expect("Could not flush stderr");
}

enum MetaFlow {
	Ignore,
	Continue,
	Break
}
struct MetaHandler {
	pub multiline_mode: bool
}
impl MetaHandler {
	pub fn new() -> Self {
		MetaHandler {
			multiline_mode: true
		}
	}

	pub fn handle(&mut self, input: &str) -> MetaFlow {
		match input {
			",exit" => MetaFlow::Break,
			",help" => {
				eprintln!("Meta commands:\n,exit\texit repl\n,mline\ttoggle multiline mode\n,help\tthis help");
				MetaFlow::Ignore
			}
			",mline" => {
				self.multiline_mode = !self.multiline_mode;
				eprintln!("Multiline: {}", self.multiline_mode);

				MetaFlow::Ignore
			}
			_ => MetaFlow::Continue
		}
	}
}

pub fn repl() {
	let stdin = std::io::stdin();
	let stderr = std::io::stderr();

	let mut meta_handler = MetaHandler::new();
	let mut buffer = String::new();

	repl_prompt(&stderr);
	for line in stdin.lock().lines() {
		// Ignore read errors
		let line = match line {
			Err(err) => {
				eprintln!("Could not read line: {}", err);
				repl_prompt(&stderr);
				continue
			}
			Ok(line) => line
		};

		// Meta commands
		match meta_handler.handle(&line) {
			MetaFlow::Break => break,
			MetaFlow::Ignore => {
				repl_prompt(&stderr);
				continue
			}
			MetaFlow::Continue => ()
		}

		if meta_handler.multiline_mode {
			if line.len() == 0 {
				crate::process_input_str(&buffer);
				buffer.clear();
			} else {
				buffer.push_str(&line);
				buffer.push_str("\n");
			}
		} else {
			crate::process_input_str(&line);
		}

		repl_prompt(&stderr);
	}
}
