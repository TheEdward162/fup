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
		Rule::FupTerm => parse_term_like(inner),

		_ => unreachable!("{:?}", inner.as_rule())
	}
}

/// Parses rules that are gramatically subsets of the `FupTerm` rule.
///
/// This is true for the `FupTerm` rule itself as well as for `Callable` and `Indexable` rules.
pub fn parse_term_like(pair: Pair) -> GrammarTree {
	let inner = pair.into_inner().nth(0).unwrap();

	match inner.as_rule() {
		Rule::FupQuote => GrammarTree::Atom(inner.as_str()),

		Rule::FupDefine => define::parse_define(inner),
		Rule::FupLetAny => let_any::parse_let_any(inner),

		Rule::OperatorExpression => operator::parse_operator_expression(inner),
		Rule::FupExpression => parse_fup_expression(inner),

		Rule::FupCall => call::parse_call(inner),
		Rule::FupIndex => index::parse_index(inner),
		Rule::FupCond => cond::parse_cond(inner),
		Rule::FupList => list::parse_list(inner),

		Rule::SchemeExpression => scheme::parse_scheme_expression(inner),

		Rule::Number | Rule::Boolean | Rule::Character | Rule::String | Rule::Name => inner.into(),

		_ => unreachable!("{:?}", inner.as_rule())
	}
}
