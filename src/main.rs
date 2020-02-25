use pest::Parser;
use pest_derive::Parser;
use std::iter;

#[derive(Parser)]
#[grammar = "grammar/fup.pest"]
pub struct FupParser;

#[derive(Debug)]
pub enum Scheme<'i> {
	Atom(&'i str),
	List(Vec<Scheme<'i>>),
}

impl<'i> From<Pair<'i>> for Scheme<'i> {
	fn from(pair: Pair<'i>) -> Scheme<'i> {
		Scheme::Atom(pair.as_str())
	}
}

impl<'i> From<&'i str> for Scheme<'i> {
	fn from(string: &'i str) -> Scheme<'i> {
		Scheme::Atom(string)
	}
}

type Pair<'i> = pest::iterators::Pair<'i, Rule>;

pub fn parse_expr(pair: Pair) -> Option<Scheme> {
	match pair.as_rule() {
		Rule::fexpr => Some(fup::parse_fexpr(pair)),
		Rule::sexpr => Some(scheme::parse_sexpr(pair)),
		Rule::atom => Some(pair.into()),
		Rule::EOI => None,
		_ => unreachable!(),
	}
}

mod fup {
	use super::*;

	pub fn parse_fexpr(pair: Pair) -> Scheme {
		assert_eq!(pair.as_rule(), Rule::fexpr);
		let pair = pair.into_inner().next().unwrap();
		match pair.as_rule() {
			Rule::define => parse_define(pair),
			_ => unreachable!(),
		}
	}

	fn parse_define(pair: Pair) -> Scheme {
		assert_eq!(pair.as_rule(), Rule::define);
		let mut pairs = pair.into_inner();
		let ident = pairs.next().unwrap().into();
		let next = pairs.next().unwrap(); // either value or params
		match next.as_rule() {
			Rule::value => Scheme::List(vec![ ident, next.into() ]),
			Rule::params => Scheme::List(
				iter::once("define".into()) // (define
				.chain(iter::once(
					// (fname params)
					Scheme::List(
						iter::once(ident)
						.chain(
							next.into_inner()
							.map(Scheme::from)
						)
						.collect()
					),
				))
				.chain(
					pairs.next().unwrap() // body)
					.into_inner()
					.filter_map(parse_expr)
				)
				.collect()
			),
			_ => unreachable!(),
		}
	}
}

mod scheme {
	use super::*;

	pub fn parse_sexpr(pair: Pair) -> Scheme {
		assert_eq!(pair.as_rule(), Rule::sexpr);
		Scheme::List(
			pair.into_inner()
				.map(|pair| match pair.as_rule() {
					Rule::sexpr => parse_sexpr(pair),
					Rule::atom => parse_atom(pair),
					_ => unreachable!(),
				})
				.collect()
		)
	}

	fn parse_atom(pair: Pair) -> Scheme {
		Scheme::Atom(pair.as_str())
	}
}


fn main() {
	use std::io::BufRead;
	let stdin = std::io::stdin();
	let mut buf = String::new();
	for line in stdin.lock().lines() {
		let line = line.unwrap();
		if line.len() == 0 {
			if buf.len() == 0 {
				break
			}
			match FupParser::parse(Rule::file, &buf) {
				Ok(spans) => println!(
					"{:#?}",
					spans.into_iter()
						.filter_map(parse_expr)
						.collect::<Vec<_>>()
				),
				Err(err) => println!("{}", err),
			}
			println!();
			buf.clear();
		} else {
			buf.push_str(&line);
			buf.push_str("\n");
		}
	}
}
