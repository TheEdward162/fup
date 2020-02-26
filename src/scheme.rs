use crate::{grammar::GrammarTree, Pair, Rule};

pub fn parse_scheme_expression(pair: Pair) -> GrammarTree {
	assert_eq!(pair.as_rule(), Rule::SchemeExpression);

	pair.into_inner()
		.map(|pair| match pair.as_rule() {
			Rule::SchemeExpression => parse_scheme_expression(pair), /* FIXME: Should be AugmentedExpression */
			Rule::Atom => pair.into(),
			_ => unreachable!()
		})
		.collect::<Vec<_>>()
		.into()
}
