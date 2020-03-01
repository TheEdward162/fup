use std::iter;

use crate::{grammar::GrammarTree, Pair, Rule};

pub fn parse_call(pair: Pair) -> GrammarTree {
	assert_eq!(pair.as_rule(), Rule::FupCall);

	let mut pairs = pair.into_inner();

	let callable: GrammarTree = parse_callable(pairs.next().unwrap());
	let arguments = pairs.next().unwrap();

	// callable(arguments..) => (callable arguments..)
	let expression: Vec<_> = iter::once(callable)
		.chain(arguments.into_inner().map(super::parse_operator_expression))
		.collect();

	expression.into()
}

fn parse_callable(pair: Pair) -> GrammarTree {
	assert_eq!(pair.as_rule(), Rule::Callable);

	let inner = pair.into_inner().nth(0).unwrap();

	match inner.as_rule() {
		Rule::OperatorExpression => super::parse_operator_expression(inner),
		Rule::SchemeExpression => super::scheme::parse_scheme_expression(inner),
		Rule::Name => inner.into(),

		_ => unreachable!("{:?}", inner.as_rule())
	}
}
