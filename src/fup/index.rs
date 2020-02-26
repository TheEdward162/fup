use std::iter;

use crate::{grammar::GrammarTree, Pair, Rule};

pub fn parse_index(pair: Pair) -> GrammarTree {
	assert_eq!(pair.as_rule(), Rule::FupIndex);

	let mut pairs = pair.into_inner();

	let name = pairs.next().unwrap().as_str();

	let index_range = pairs.next().unwrap();
	match index_range.as_rule() {
		// name[start .. end] => (list (car (cdr (cdr name))) ... (car ... (cdr name)))
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
					.chain(index_until(name, end_index).skip(start_index))
					.collect::<Vec<_>>()
					.into()
			}
		}
		// name[.. end] => (list (car name) (car (cdr name)) ... (car .. (cdr name)))
		Rule::FupIndexOpenLeft => {
			let end_index: usize =
				index_range.into_inner().nth(0).unwrap().as_str().parse().unwrap();

			iter::once(GrammarTree::Atom("list"))
				.chain(index_until(name, end_index))
				.collect::<Vec<_>>()
				.into()
		}
		// name[start ..] => (cdr .. (cdr name))
		Rule::FupIndexOpenRight => {
			let start_index: usize =
				index_range.into_inner().nth(0).unwrap().as_str().parse().unwrap();

			index_after(name, start_index)
		}
		// name[index] => (car (cdr .. (cdr name)))
		Rule::FupIndexExact => {
			let exact_index: usize =
				index_range.into_inner().nth(0).unwrap().as_str().parse().unwrap();

			vec![GrammarTree::Atom("car"), index_after(name, exact_index)].into()
		}

		_ => unreachable!()
	}
}

fn index_after(name: &str, start_index: usize) -> GrammarTree {
	(0 .. start_index).fold(GrammarTree::Atom(name), |expr, _| vec!["cdr".into(), expr].into())
}

fn index_until(name: &str, end_index: usize) -> impl Iterator<Item = GrammarTree> {
	// Explicitly moving a reference.. that's the brwchkr I know
	(0 .. end_index)
		.map(move |index| vec![GrammarTree::Atom("car"), index_after(name, index)].into())
}
