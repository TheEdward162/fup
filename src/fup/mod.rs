mod call;
mod cond;
mod define;
mod operator;
mod index;
mod let_any;
mod list;
mod scheme;

pub use operator::parse_operator_expression;

use crate::{grammar::GrammarTree, Pair, Rule};

pub fn parse_fup_expression(pair: Pair) -> GrammarTree {
	assert_eq!(pair.as_rule(), Rule::FupExpression);
	
	let inner = pair.into_inner().nth(0).unwrap();

	match inner.as_rule() {
		Rule::FupTerm => parse_fup_term(inner),
		
		Rule::Name => inner.into(), // Name is a subset of Identifier, because Identifiers (like `get-x`) are be ambiguous in FUP.

		_ => unreachable!("{:?}", inner.as_rule())
	}
}

pub fn parse_fup_term(pair: Pair) -> GrammarTree {
	assert_eq!(pair.as_rule(), Rule::FupTerm);
	
	let inner = pair.into_inner().nth(0).unwrap();

	match inner.as_rule() {
		Rule::FupList => list::parse_list(inner),
		Rule::FupQuote => GrammarTree::Atom(inner.as_str()),

		Rule::FupLetAny => let_any::parse_let_any(inner),
		Rule::FupDefine => define::parse_define(inner),
		Rule::FupCond => cond::parse_cond(inner),

		Rule::FupIndex => index::parse_index(inner),
		Rule::FupCall => call::parse_call(inner),

		Rule::SchemeExpression => scheme::parse_scheme_expression(inner),
		Rule::OperatorExpression => operator::parse_operator_expression(inner),

		Rule::Number | Rule::Boolean | Rule::Character | Rule::String => inner.into(),

		_ => unreachable!("{:?}", inner.as_rule())
	}
}
