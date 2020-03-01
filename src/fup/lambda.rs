use crate::{grammar::GrammarTree, Pair, Rule};

pub fn parse_lambda(pair: Pair) -> GrammarTree {
	assert_eq!(pair.as_rule(), Rule::FupLambda);

	let mut pairs = pair.into_inner();
	let parameters = pairs.next().unwrap()
        .into_inner()
        .map(GrammarTree::from)
        .collect::<Vec<_>>()
        .into();
    let body = super::parse_operator_expression(pairs.next().unwrap());

    vec!["lambda".into(), parameters, body].into()
}
