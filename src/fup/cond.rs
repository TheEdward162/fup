use std::iter;

use crate::{grammar::GrammarTree, Pair, Rule};

pub fn parse_cond(pair: Pair) -> GrammarTree {
	assert_eq!(pair.as_rule(), Rule::FupCond);

	// cond { a => b, .. } => (cond (a b) ..)
	iter::once("cond".into())
		.chain(pair.into_inner().map(|arm| {
			arm.into_inner()
				.map(super::parse_operator_expression)
				.collect::<Vec<_>>()
				.into()
		}))
		.collect::<Vec<_>>()
		.into()
}
