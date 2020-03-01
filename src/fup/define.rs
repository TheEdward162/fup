use std::iter;

use crate::{grammar::GrammarTree, Pair, Rule};

pub fn parse_define(pair: Pair) -> GrammarTree {
	assert_eq!(pair.as_rule(), Rule::FupDefine);

	let mut pairs = pair.into_inner();

	let name = pairs.next().unwrap().into();

	let next = pairs.next().unwrap(); // Either `OperatorExpression` or `FnParameters`
	if next.as_rule() == Rule::OperatorExpression {
		// define name = value; => (define name value)
		return vec![name, super::parse_operator_expression(next)].into()
	}

	let parameters = next;
	// define name(parameters..) { body } => (define (name parameters..) body)
	let expression: Vec<_> = iter::once(GrammarTree::Atom("define"))
		.chain(iter::once(GrammarTree::List(
			iter::once(name)
				.chain(parameters.into_inner().map(GrammarTree::from))
				.collect()
		)))
		.chain(pairs.map(super::parse_operator_expression))
		.collect();

	expression.into()
}
