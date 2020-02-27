use std::iter;

use crate::{fup, grammar::GrammarTree, Pair, Rule};

pub fn parse_call(pair: Pair) -> GrammarTree {
	assert_eq!(pair.as_rule(), Rule::FupCall);

	let mut pairs = pair.into_inner();

	let callable: GrammarTree = super::parse_term_like(pairs.next().unwrap());
	let arguments = pairs.next().unwrap();

	// callable(arguments..) => (callable arguments..)
	let expression: Vec<_> = iter::once(callable)
		.chain(arguments.into_inner().map(fup::parse_fup_expression))
		.collect();

	expression.into()
}
