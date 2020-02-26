use super::parse_input_str;

macro_rules! grammar_tree {
	(
		()
	) => {
		$crate::grammar::GrammarTree::List(
			Vec::new()
		)
	};
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
	r#"
		( ; Compute fibonacci number
			define (fib n)
			(cond
				((= n 0) 0)
				((= n 1) 1)
				(#t (
					+
					(fib (- n 1))
					(fib (- n 2))
				))
			)
		)

		(fib 10)
	"#,
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
	r#"
		define FOO(x) {
			(+ x 1)
		}

		(display (FOO 1))

		(define (FOO x) (+ x 1))
	"#,
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
	index_exact,
	r#"
		a[1]
	"#,
	("car" ("cdr" "a"))
}
test! {
	index_right_open,
	r#"
		b[2..]
	"#,
	("cdr" ("cdr" "b"))
}
test! {
	index_left_open,
	r#"
		c[..2]
	"#,
	(
		"list"
		("car" "c")
		("car" ("cdr" "c"))
	)
}
test! {
	index_full_empty,
	r#"
		d[1..1]
	"#,
	()
}
test! {
	index_full,
	r#"
		e[1..3]
	"#,
	(
		"list"
		("car" ("cdr" "e"))
		("car" ("cdr" ("cdr" "e")))
	)
}

test! {
	cond,
	r#"
		cond {
			(= 1 2) => (display 1),
			(= 1 3) => (display 2),
			else => (display 3)
		}

		(cond ((= 1 2) (display 1)) ((= 1 3) (display 2)) (else (display 3)))
	"#,
	("cond"
		(("=" "1" "2") ("display" "1"))
		(("=" "1" "3") ("display" "2"))
		("else" ("display" "3"))
	)
	("cond"
		(("=" "1" "2") ("display" "1"))
		(("=" "1" "3") ("display" "2"))
		("else" ("display" "3"))
	)
}

test! {
	infix,
	r#"
		6 * 7
		1 + 2 - 3 / 2
		a : b : c
		a : (cons b c)
		{1 + 2} * 3
	"#,
	("*" "6" "7")
	("-" ("+" "1" "2") ("/" "3" "2"))
	("cons" "a" ("cons" "b" "c"))
	("cons" "a" ("cons" "b" "c"))
	("*" ("+" "1" "2") "3")
}

test! {
	mixed,
	r#"
		(+ 1 a(b))
		(list a[0] a[1])
		(+ 1 { 6 * 7 })
		(foo cond { a => b, c => d })
	"#,
	("+" "1" ("a" "b"))
	("list"
		("car" "a")
		("car" ("cdr" "a")))
	("+" "1" ("*" "6" "7"))
	("foo"
		("cond" ("a" "b")
				("c" "d")))
}
