use crate::{grammar::GrammarTree, Pair, Rule};

pub fn parse_scheme_expression(pair: Pair) -> GrammarTree {
	assert_eq!(pair.as_rule(), Rule::SchemeExpression);

	pair.into_inner()
		.map(|pair| match pair.as_rule() {
            Rule::FupTerm => crate::fup::parse_term(pair),
			Rule::SchemeExpression => parse_scheme_expression(pair),
			Rule::Atom => pair.into(),
			_ => unreachable!()
		})
		.collect::<Vec<_>>()
		.into()
}
