use pest::prec_climber::PrecClimber;

use crate::{grammar::GrammarTree, Pair, Rule};

macro_rules! assoc {
	( L ) => {
		pest::prec_climber::Assoc::Left
	};
	( R ) => {
		pest::prec_climber::Assoc::Right
	};
}

macro_rules! climber {
	(
		$(
			$assoc:ident
			$rule:ident $( | $rules:ident )* ,
		)+
	) => {
		pest::prec_climber::PrecClimber::new(
			vec![
				$(
					pest::prec_climber::Operator::new(Rule::$rule, assoc!($assoc))
					$( | pest::prec_climber::Operator::new(Rule::$rules, assoc!($assoc)) )*
				),+
			]
		);
	};
}

fn climber() -> &'static PrecClimber<Rule> {
	use std::{mem::MaybeUninit, sync::Once};

	static GUARD: Once = Once::new();
	static mut CLIMBER: MaybeUninit<PrecClimber<Rule>> = MaybeUninit::uninit();

	GUARD.call_once(|| {
		// Safety: writing to a MaybeUninit is safe when guarded by Once
		unsafe {
			CLIMBER.as_mut_ptr().write(climber![
				L OpAppend,
				R OpCons,
				L OpOr,
				L OpAnd,
				L OpLowerThanOrEqual | OpGreaterThanOrEqual,
				L OpLowerThen | OpGreaterThen,
				L OpAdd | OpSubtract,
				L OpMultiply | OpDivide | OpModulo,
				L OpEq,
				L OpEqv,
				L OpEqual,
				L OpMathEqual,
			]);
		}
	});

	// Safety: immutable reference, prior initialization guaranteed by Once
	unsafe { CLIMBER.as_ptr().as_ref().unwrap() }
}

pub fn parse_fup_expression(pair: Pair) -> GrammarTree {
	assert_eq!(pair.as_rule(), Rule::FupExpression);
	climber().climb(pair.into_inner(), climber_term, climber_infix)
}

pub fn parse_term(pair: Pair) -> GrammarTree {
	let inner = pair.into_inner().nth(0).unwrap();
	match inner.as_rule() {
		Rule::FupExpression => parse_fup_expression(inner),
		Rule::FupCall => crate::fup::call::parse_call(inner),
		Rule::FupIndex => crate::fup::index::parse_index(inner),
		Rule::FupCond => crate::fup::cond::parse_cond(inner),
		Rule::SchemeExpression => crate::scheme::parse_scheme_expression(inner),
		Rule::Number | Rule::Boolean | Rule::Character | Rule::String | Rule::Name => inner.into(),

		_ => unreachable!()
	}
}

fn climber_infix<'i>(lhs: GrammarTree<'i>, op: Pair<'i>, rhs: GrammarTree<'i>) -> GrammarTree<'i> {
	vec![parse_infix_operator(op), lhs, rhs].into()
}

fn climber_term(pair: Pair) -> GrammarTree {
	match pair.as_rule() {
		Rule::FupDefine => super::define::parse_define(pair),
		Rule::FupTerm => parse_term(pair),

		_ => unreachable!()
	}
	.into()
}

fn parse_infix_operator(pair: Pair) -> GrammarTree<'static> {
	match pair.as_rule() {
		Rule::OpAppend => "append",
		Rule::OpCons => "cons",
		Rule::OpAdd => "+",
		Rule::OpSubtract => "-",
		Rule::OpMultiply => "*",
		Rule::OpDivide => "/",
		Rule::OpModulo => "%",
		Rule::OpAnd => "and",
		Rule::OpOr => "or",
		Rule::OpLowerThanOrEqual => "<=",
		Rule::OpGreaterThanOrEqual => ">=",
		Rule::OpLowerThen => "<",
		Rule::OpGreaterThen => ">",
		Rule::OpEq => "eq?",
		Rule::OpEqv => "eqv?",
		Rule::OpEqual => "equal?",
		Rule::OpMathEqual => "=",

		_ => unreachable!()
	}
	.into()
}
