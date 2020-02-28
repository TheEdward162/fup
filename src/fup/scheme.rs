use crate::{grammar::GrammarTree, Pair, Rule};

pub fn parse_scheme_expression(pair: Pair) -> GrammarTree {
	assert_eq!(pair.as_rule(), Rule::SchemeExpression);

	pair.into_inner()
		.map(|pair| match pair.as_rule() {
			Rule::FupTerm => super::parse_term_like(pair),
			Rule::SchemeExpression => parse_scheme_expression(pair),
			Rule::Atom => pair.into(),
			_ => unreachable!("{:?}", pair.as_rule())
		})
		.collect::<Vec<_>>()
		.into()
}
