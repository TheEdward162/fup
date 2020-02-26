use std::iter;

use crate::{grammar::GrammarTree, Pair, Rule};

pub fn parse_fup_expression(pair: Pair) -> GrammarTree {
	assert_eq!(pair.as_rule(), Rule::FupExpression);

	let inner = pair.into_inner().nth(0).unwrap();
	match inner.as_rule() {
		Rule::FupDefine => parse_define(inner),
		Rule::FupCall => parse_call(inner),
		Rule::FupCond => parse_cond(inner),
		Rule::FupIndex => parse_index(inner),
		// TODO infix operators, brackets, let, ...
		_ => unreachable!()
	}
	.into()
}

fn parse_define(pair: Pair) -> GrammarTree {
	assert_eq!(pair.as_rule(), Rule::FupDefine);

	let mut pairs = pair.into_inner();

	let name = pairs.next().unwrap().into();

	let next = pairs.next().unwrap(); // Either `Expression` or `FnParameters`
	if next.as_rule() == Rule::AugmentedExpression {
		// define name = value; => (define name value)
		return vec![name, crate::parse_augmented_expression(next)].into()
	}

	let parameters = next;
	// define name(parameters..) { body } => (define (name parameters..) body)
	let expression: Vec<_> = iter::once(GrammarTree::Atom("define"))
		.chain(iter::once(GrammarTree::List(
			iter::once(name).chain(parameters.into_inner().map(GrammarTree::from)).collect()
		)))
		.chain(pairs.next().unwrap().into_inner().map(crate::parse_augmented_expression))
		.collect();

	expression.into()
}

fn parse_call(pair: Pair) -> GrammarTree {
	assert_eq!(pair.as_rule(), Rule::FupCall);

	let mut pairs = pair.into_inner();

	let name = pairs.next().unwrap();
	let arguments = pairs.next().unwrap();

	// name(arguments..) => (name arguments..)
	let expression: Vec<_> = iter::once(name.into())
		.chain(arguments.into_inner().map(crate::parse_augmented_expression))
		.collect();

	expression.into()
}

fn parse_cond(pair: Pair) -> GrammarTree {
	assert_eq!(pair.as_rule(), Rule::FupCond);

	// cond { a => b, .. } => (cond (a b) ..)
	iter::once("cond".into())
		.chain(pair.into_inner().map(|arm| {
			arm.into_inner().map(crate::parse_augmented_expression).collect::<Vec<_>>().into()
		}))
		.collect::<Vec<_>>()
		.into()
}

fn parse_index(pair: Pair) -> GrammarTree {
	assert_eq!(pair.as_rule(), Rule::FupIndex);

	let mut pairs = pair.into_inner();

	let name = pairs.next().unwrap().into();
	let index = pairs.next().unwrap().as_str().parse().unwrap(); // parse into number
	let has_tail = pairs.next().is_some();

	// name[index (..)?] => (car? (cdr .. (cdr name)))
	let expression = (0 .. index).fold(name, |inner, _| vec!["cdr".into(), inner].into());

	if !has_tail {
		vec!["car".into(), expression].into()
	} else {
		expression
	}
}
