use pest::Parser;
use pest_derive::Parser;

mod fup;
mod grammar;

mod repl;

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
		Err(e) => {
			eprintln!("{}", e);
			if cfg!(debug) {
				eprintln!("{:?}", e);
			}
		}
		Ok(tree) => println!("{}", tree)
	}
}


fn main() {
	let mut cli = std::env::args();
	if cli.len() > 1 {
		let file = cli.nth(1).unwrap();
		let input = std::fs::read_to_string(file).expect("Could not read file");
		process_input_str(&input);
	} else {
		repl::repl();
	}
}
