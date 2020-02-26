use super::parse_input_str;

macro_rules! grammar_tree {
	(
		( $( $tok: tt )+ )
	) => {
		grammar_tree!($( $tok )+)
	};
	(
		$atom: literal
	) => {
		$crate::grammar::GrammarTree::Atom($atom)
	};
	(
		$( $tok: tt )+
	) => {
		$crate::grammar::GrammarTree::List(
			vec![
				$(
					grammar_tree!($tok)
				),+
			]
		)
	};
}

macro_rules! test {
	(
		$test_name: ident,
		$input_code: expr,
		$( $output_code: tt )+
	) => {
		#[test]
		fn $test_name() {
			let parsed = match parse_input_str($input_code) {
				Ok(code) => code,
				Err(err) => panic!("{}", err),
			};
			let expected = $crate::grammar::GrammarTree::List(
				vec![
					$(
						grammar_tree!($output_code)
					),+
				]
			);

			if parsed != expected {
				panic!(
					"\nEXPECTED:{}\n\nPARSED:\n{}\n\n",
					expected,
					parsed,
				)
			}
		}
	};
}

test! {
	scheme,
	include_str!("tests/scheme.scm"),
	(
		"define" ("fib" "n")
		("cond"
		 (("=" "n" "0") "0")
		 (("=" "n" "1") "1")
		 ("#t" (
				 "+"
				 ("fib" ("-" "n" "1"))
				 ("fib" ("-" "n" "2"))
		 ))
		)
	)

	("fib" "10")
}

test! {
	define,
	include_str!("tests/define.scm"),
	(
		"define"
		("FOO" "x")
		("+" "x" "1")
	)
		(
			"display"
			("FOO" "1")
		)
		(
			"define"
			("FOO" "x")
			("+" "x" "1")
		)
}

test! {
	call,
	r#"
		func(arg1, arg2)
	"#,
	("func" "arg1" "arg2")
}

test! {
	index,
	r#"
		a[1]
		b[2..]
	"#,
	("car" ("cdr" "a"))
	("cdr" ("cdr" "b"))
}

test! {
	cond,
	include_str!("tests/cond.scm"),
	("cond"
		(("=" "1" "2") ("display" "1"))
		(("=" "1" "3") ("display" "2"))
		("else" ("display" "3"))
	)
}
