use std::fmt::Write;

use pest::iterators::Pair;

use crate::Rule;

/// Handles pairs iterator, accumulating output into `buffer`.
pub fn handle_pairs<'i, I>(pairs: I, buffer: &mut String) -> Result<(), std::fmt::Error> where
	I: Iterator<Item = Pair<'i, Rule>>
{
	for pair in pairs {
		match pair.as_rule() {
			Rule::aug_definition => handle_aug_definition(pair, buffer)?,
			_ => write!(buffer, " {}", pair.as_str())?
		}
	}

	Ok(())
}

/// Handle
/// ```
/// define NAME(FIRST, SECOND, ...) {
/// 	; ...
/// }
/// ```
/// =>
/// ```
/// (
/// 	define (NAME FIRST SECOND ...)
/// 		; ...
/// )
/// ```
fn handle_aug_definition<'i>(pair: Pair<'i, Rule>, buffer: &mut String) -> Result<(), std::fmt::Error> {
	let mut pairs = pair.into_inner().peekable();

	let name = pairs.next().unwrap().as_str();
	write!(buffer, "(define ({}", name)?;

	loop {
		let peek = pairs.peek();
		match peek.map(|p| p.as_rule()) {
			Some(Rule::variable) => (),
			_ => break
		}

		let variable = pairs.next().unwrap();
		write!(buffer, " {}", variable.as_str())?;
	}

	write!(buffer, ") ")?;

	// Handle the inside
	handle_pairs(pairs, buffer)?;

	// TODO: This could have been tail recursion
	write!(buffer, ")")?;
	Ok(())
}