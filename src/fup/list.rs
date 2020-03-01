use std::iter;

use crate::{grammar::GrammarTree, Pair, Rule};

pub fn parse_list(pair: Pair) -> GrammarTree {
	assert_eq!(pair.as_rule(), Rule::FupList);

	// [ a, b, .. ] => (list a b ..)
	iter::once("list".into())
		.chain(pair.into_inner().map(super::parse_fup_expression))
		.collect::<Vec<_>>()
		.into()
}
