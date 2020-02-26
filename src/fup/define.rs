use std::iter;

use crate::{grammar::GrammarTree, Pair, Rule};

pub fn parse_define(pair: Pair) -> GrammarTree {
	assert_eq!(pair.as_rule(), Rule::FupDefine);

	let mut pairs = pair.into_inner();

	let name = pairs.next().unwrap().into();

	let next = pairs.next().unwrap(); // Either `Expression` or `FnParameters`
	if next.as_rule() == Rule::AugmentedExpression {
		// define name = value; => (define name value)
		return vec![name, crate::parse_augmented_expression(next)].into()
	}

	let parameters = next;
	// define name(parameters..) { body } => (define (name parameters..) body)
	let expression: Vec<_> = iter::once(GrammarTree::Atom("define"))
		.chain(iter::once(GrammarTree::List(
			iter::once(name).chain(parameters.into_inner().map(GrammarTree::from)).collect()
		)))
		.chain(pairs.next().unwrap().into_inner().map(crate::parse_augmented_expression))
		.collect();

	expression.into()
}
