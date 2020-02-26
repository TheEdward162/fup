use pest::Parser;
use pest_derive::Parser;

mod fup;
mod grammar;
mod scheme;

#[cfg(test)]
mod tests;

#[derive(Parser)]
#[grammar = "fup.pest"]
pub struct FupParser;

type Pair<'i> = pest::iterators::Pair<'i, Rule>;

fn parse_input_str(input: &str) -> Result<grammar::GrammarTree, pest::error::Error<Rule>> {
	let pairs = FupParser::parse(Rule::Input, input)?;

	let tree = grammar::GrammarTree::List(
		pairs
			.into_iter()
			.filter(|p| p.as_rule() != Rule::EOI)
			.map(fup::parse_fup_expression)
			.collect()
	);

	Ok(tree)
}

fn process_input_str(input: &str) {
	match parse_input_str(input) {
		Err(e) => eprintln!("{}", e),
		Ok(tree) => println!("{}", tree)
	}
}

fn repl() {
	use std::io::BufRead;

	let stdin = std::io::stdin();

	let mut multiline_mode = true;
	let mut buffer = String::new();

	for line in stdin.lock().lines() {
		// Ignore read errors
		let line = match line {
			Err(_) => continue,
			Ok(line) => line
		};

		// Meta commands
		if line == ",exit" {
			break
		} else if line == ",mline" {
			multiline_mode = !multiline_mode;
			eprintln!("Multiline: {}", multiline_mode);
		}

		if multiline_mode {
			if line.len() == 0 {
				process_input_str(&buffer);
				buffer.clear();
			} else {
				buffer.push_str(&line);
				buffer.push_str("\n");
			}
		} else {
			process_input_str(&line);
		}
	}
}

fn main() {
	let mut cli = std::env::args();
	if cli.len() > 1 {
		let file = cli.nth(1).unwrap();
		let input = std::fs::read_to_string(file).expect("Could not read file");
		process_input_str(&input);
	} else {
		repl();
	}
}
