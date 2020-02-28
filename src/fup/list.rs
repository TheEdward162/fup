use std::iter;

use crate::{fup, grammar::GrammarTree, Pair, Rule};

pub fn parse_list(pair: Pair) -> GrammarTree {
	assert_eq!(pair.as_rule(), Rule::FupList);

	// [ a, b, .. ] => (list a b ..)
	iter::once("list".into())
		.chain(pair.into_inner().map(fup::parse_term_like))
		.collect::<Vec<_>>()
		.into()
}
