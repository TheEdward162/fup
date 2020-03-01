use std::iter;

use crate::{grammar::GrammarTree, Pair, Rule};

pub fn parse_let_any(pair: Pair) -> GrammarTree {
	assert_eq!(pair.as_rule(), Rule::FupLetAny);

	let let_inner = pair.into_inner().nth(0).unwrap();

	let let_type: GrammarTree = match let_inner.as_rule() {
		Rule::FupLetStar => "let*",
		Rule::FupLetRec => "letrec",
		Rule::FupLet => "let",

		_ => unreachable!("{:?}", let_inner.as_rule())
	}
	.into();

	let mut pairs = let_inner.into_inner();

	let assignments = parse_assignments(pairs.next().unwrap());

	GrammarTree::List(
		iter::once(let_type)
			.chain(iter::once(assignments))
			.chain(pairs.map(super::parse_operator_expression))
			.collect()
	)
}

/// Parse `foo = X, bar = Y, baz = Z` into `((foo X) (bar Y) (baz Z))`
fn parse_assignments(pair: Pair) -> GrammarTree {
	assert_eq!(pair.as_rule(), Rule::FupLetAssignments);

	let mut list = Vec::new();

	let mut assignments = pair;
	loop {
		let mut inner = assignments.into_inner();

		let first = inner.next();
		if first.is_none() {
			break
		}

		let name = first.unwrap();
		let expression = inner.next().unwrap();

		let tree = GrammarTree::List(vec![
			name.into(),
			super::parse_operator_expression(expression),
		]);

		list.push(tree);

		match inner.next() {
			Some(ass) => {
				assignments = ass;
			}
			None => break
		}
	}

	list.into()
}
