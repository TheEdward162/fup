use crate::{grammar::GrammarTree, Pair, Rule};

mod define;
mod call;
mod cond;
mod index;

pub fn parse_fup_expression(pair: Pair) -> GrammarTree {
	assert_eq!(pair.as_rule(), Rule::FupExpression);

	let inner = pair.into_inner().nth(0).unwrap();
	match inner.as_rule() {
		Rule::FupDefine => define::parse_define(inner),
		Rule::FupCall => call::parse_call(inner),
		Rule::FupCond => cond::parse_cond(inner),
		Rule::FupIndex => index::parse_index(inner),
		// TODO infix operators, brackets, let, ...
		_ => unreachable!()
	}
	.into()
}

