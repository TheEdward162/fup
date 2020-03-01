use std::iter;

use crate::{grammar::GrammarTree, Pair, Rule};

pub fn parse_index(pair: Pair) -> GrammarTree {
	assert_eq!(pair.as_rule(), Rule::FupIndex);

	let mut pairs = pair.into_inner();

	let indexable: GrammarTree = parse_indexable(pairs.next().unwrap());

	let index_range = pairs.next().unwrap();
	match index_range.as_rule() {
		// indexable[start .. end] => (list (car (cdr (cdr indexable))) ... (car ... (cdr indexable)))
		Rule::FupIndexFull => {
			let (start_index, end_index): (usize, usize) = {
				let mut range = index_range.into_inner();
				(
					range.next().unwrap().as_str().parse().unwrap(),
					range.next().unwrap().as_str().parse().unwrap()
				)
			};

			if start_index >= end_index {
				GrammarTree::List(vec![])
			} else {
				iter::once(GrammarTree::Atom("list"))
					.chain(index_until(indexable, end_index).skip(start_index))
					.collect::<Vec<_>>()
					.into()
			}
		}
		// indexable[.. end] => (list (car indexable) (car (cdr indexable)) ... (car .. (cdr indexable)))
		Rule::FupIndexOpenLeft => {
			let end_index: usize = index_range
				.into_inner()
				.nth(0)
				.unwrap()
				.as_str()
				.parse()
				.unwrap();

			iter::once(GrammarTree::Atom("list"))
				.chain(index_until(indexable, end_index))
				.collect::<Vec<_>>()
				.into()
		}
		// indexable[start ..] => (cdr .. (cdr indexable))
		Rule::FupIndexOpenRight => {
			let start_index: usize = index_range
				.into_inner()
				.nth(0)
				.unwrap()
				.as_str()
				.parse()
				.unwrap();

			index_after(indexable, start_index)
		}
		// indexable[index] => (car (cdr .. (cdr indexable)))
		Rule::FupIndexExact => {
			let exact_index: usize = index_range
				.into_inner()
				.nth(0)
				.unwrap()
				.as_str()
				.parse()
				.unwrap();

			vec![
				GrammarTree::Atom("car"),
				index_after(indexable, exact_index),
			]
			.into()
		}

		_ => unreachable!("{:?}", index_range.as_rule())
	}
}

fn index_after(indexable: GrammarTree, start_index: usize) -> GrammarTree {
	(0 .. start_index).fold(indexable, |expr, _| vec!["cdr".into(), expr].into())
}

fn index_until(indexable: GrammarTree, end_index: usize) -> impl Iterator<Item = GrammarTree> {
	(0 .. end_index).map(move |index| {
		vec![
			GrammarTree::Atom("car"),
			index_after(indexable.clone(), index),
		]
		.into()
	})
}

fn parse_indexable(pair: Pair) -> GrammarTree {
	assert_eq!(pair.as_rule(), Rule::Indexable);
	
	let inner = pair.into_inner().nth(0).unwrap();

	match inner.as_rule() {
		Rule::OperatorExpression => super::parse_operator_expression(inner),
		Rule::SchemeExpression => super::scheme::parse_scheme_expression(inner),
		Rule::Name => inner.into(),

		_ => unreachable!("{:?}", inner.as_rule())
	}
}