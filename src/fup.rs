use std::iter;

use pest::prec_climber::PrecClimber;

use crate::{grammar::GrammarTree, Pair, Rule};

macro_rules! assoc {
    ( L ) => { pest::prec_climber::Assoc::Left };
    ( R ) => { pest::prec_climber::Assoc::Right };
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
    use std::mem::MaybeUninit;
    use std::sync::Once;

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
    climber().climb(
        pair.into_inner(),
        climber_term,
        climber_infix,
    )
}

fn climber_infix<'i>(lhs: GrammarTree<'i>, op: Pair<'i>, rhs: GrammarTree<'i>) -> GrammarTree<'i> {
    vec![
        parse_infix_operator(op),
        lhs,
        rhs,
    ].into()
}

fn climber_term(pair: Pair) -> GrammarTree {
	match pair.as_rule() {
        Rule::FupDefine => parse_define(pair),
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

        _ => unreachable!(),
    }.into()
}

pub fn parse_term(pair: Pair) -> GrammarTree {
	let inner = pair.into_inner().nth(0).unwrap();
	match inner.as_rule() {
		Rule::FupExpression => parse_fup_expression(inner),
		Rule::FupCall => parse_call(inner),
		Rule::FupIndex => parse_index(inner),
        Rule::FupCond => parse_cond(inner),
		Rule::SchemeExpression => crate::scheme::parse_scheme_expression(inner),
		Rule::Number | Rule::Boolean | Rule::Character | Rule::String | Rule::Name => inner.into(),

		_ => unreachable!()
	}
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
