use std::iter;

use crate::{grammar::GrammarTree, Pair, Rule};

pub fn parse_call(pair: Pair) -> GrammarTree {
	assert_eq!(pair.as_rule(), Rule::FupCall);

	let mut pairs = pair.into_inner();

	let name = pairs.next().unwrap();
	let arguments = pairs.next().unwrap();

	// name(arguments..) => (name arguments..)
	let expression: Vec<_> = iter::once(name.into())
		.chain(arguments.into_inner().map(crate::parse_augmented_expression))
		.collect();

	expression.into()
}