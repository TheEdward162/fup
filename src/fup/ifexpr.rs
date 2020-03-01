use std::iter;

use crate::{grammar::GrammarTree, Pair, Rule};

pub fn parse_if(pair: Pair) -> GrammarTree {
	assert_eq!(pair.as_rule(), Rule::FupIf);

    iter::once("if".into())
        .chain(
            pair.into_inner()
                .map(super::parse_operator_expression)
        )
        .collect::<Vec<_>>()
        .into()
}
