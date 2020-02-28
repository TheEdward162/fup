mod call;
mod cond;
mod define;
mod expression;
mod index;
mod list;
mod scheme;

pub use expression::parse_fup_expression;

use crate::{grammar::GrammarTree, Pair, Rule};

/// Parses all possible non-terminal rules and `FupTerm` terminal rules.
pub fn parse_term_like(pair: Pair) -> GrammarTree {
	let inner = pair.into_inner().nth(0).unwrap();

	match inner.as_rule() {
		Rule::FupExpression => expression::parse_fup_expression(inner),

		Rule::FupCall => call::parse_call(inner),
		Rule::FupIndex => index::parse_index(inner),
		Rule::FupCond => cond::parse_cond(inner),
		Rule::FupList => list::parse_list(inner),

		Rule::SchemeExpression => scheme::parse_scheme_expression(inner),

		Rule::Number | Rule::Boolean | Rule::Character | Rule::String | Rule::Name => inner.into(),

		_ => unreachable!("{:?}", inner.as_rule())
	}
}
