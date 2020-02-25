use pest::Parser;

use pest_derive::Parser;

mod rules;

#[derive(Parser)]
#[grammar = "grammar/fup.pest"]
pub struct SchemeParser;

fn main() {
	let args: Vec<String> = std::env::args().collect();
    eprintln!("CLI: {:?}", args);
	
	let file_path = args.get(1).expect("No file provided");

	let file = std::fs::read_to_string(file_path).expect("Could not read test file");
	let pairs = match SchemeParser::parse(
		Rule::file,
		&file
	) {
		Err(e) => {
			eprintln!("{:#?}", e);
			return;
		}
		Ok(v) => v
	};

	let mut output = String::new();
	rules::handle_pairs(pairs, &mut output).expect("Could not format output");

	println!("{}", output);
}

#[cfg(test)]
mod test {
	use pest::Parser;

	use super::SchemeParser;

	#[test]
	fn test00() {
		
	}
}