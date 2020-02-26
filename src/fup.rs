use std::iter;

use crate::{grammar::GrammarTree, Pair, Rule};

pub fn parse_fup_expression(pair: Pair) -> GrammarTree {
	assert_eq!(pair.as_rule(), Rule::FupExpression);

	let inner = pair.into_inner().nth(0).unwrap();
	match inner.as_rule() {
		Rule::FupDefine => parse_define(inner),
		// TODO: cond, list indexing, more..
		_ => unreachable!()
	}
	.into()
}

fn parse_define(pair: Pair) -> GrammarTree {
	assert_eq!(pair.as_rule(), Rule::FupDefine);

	let mut pairs = pair.into_inner();

	let name = pairs.next().unwrap().into();

	let next = pairs.next().unwrap(); // Either `Expression` or `FnArguments`
	if next.as_rule() == Rule::AugmentedExpression {
		// define name = value; => (define name value)
		return vec![name, crate::parse_augmented_expression(next)].into()
	}

	let arguments = next;
	// define name(arguments..) { body } => (define (name arguments..) body)
	let expression: Vec<_> = iter::once(GrammarTree::Atom("define"))
		.chain(iter::once(GrammarTree::List(
			iter::once(name).chain(arguments.into_inner().map(GrammarTree::from)).collect()
		)))
		.chain(pairs.next().unwrap().into_inner().map(crate::parse_augmented_expression))
		.collect();

	expression.into()
}
