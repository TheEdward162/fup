use super::parse_input_str;

macro_rules! grammar_tree {
	(
		$atom: literal
	) => {
		$crate::grammar::GrammarTree::Atom($atom)
	};
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
				let fmt_input = format!("INPUT:\n{}", $input_code);
				let fmt_expected = format!("EXPECTED:\n{}", expected);
				let fmt_parsed = format!("PARSED:\n{}", parsed);
				let fmt_tree = format!("TREE:\n{:?}", parsed);
				let message = format!(
					"\n>> {}\n\n>> {}\n\n>> {}\n\n>> {}",
					fmt_input,
					fmt_expected,
					fmt_parsed,
					fmt_tree
				);
				panic!(message)
			}
		}
	};
}

// SCHEME
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
	scheme_let,
	r#"
		(let ((x 1)) x)
	"#,
	(
		"let"
		(("x" "1"))
		"x"
	)
}
test! {
	scheme_let_2,
	r#"
		(let ((x 1) (y 1)) (+ x y))
	"#,
	(
		"let"
		(("x" "1") ("y" "1"))
		("+" "x" "y")
	)
}

// DEFINE
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
	define_with_space,
	r#"
		define FOO (x) {
			(+ x 1)
		}
	"#,
	(
		"define"
		("FOO" "x")
		("+" "x" "1")
	)
}

// LET
test! {
	let_plain,
	r#"
		let x = 1 {
			(display x)
		}
	"#,
	(
		"let"
		(
			("x" "1")
		)
		("display" "x")
	)
}
test! {
	letrec,
	r#"
		let* x = foo(1,), y = x {
			y
		}
	"#,
	(
		"let*"
		(
			("x" ("foo" "1"))
			("y" "x")
		)
		"y"
	)
}
test! {
	let_unscoped,
	r#"
		let x = 1
		(display x)
	"#,
	(
		"let"
		(
			("x" "1")
		)
		("display" "x")
	)
}

// CALL
test! {
	call,
	r#"
		func(arg1, arg2)

		func(arg,)

		func (arg,)

		func(,)
	"#,
	("func" "arg1" "arg2")
	("func" "arg")
	("func" "arg")
	("func")
}
test! {
	call_expression,
	r#"
		{ cond { #f => foo, #t => bar } }(1, 1 + 1)
		(lambda (x) x)(1,)
	"#,
	(
		("cond"
			("#f" "foo")
			("#t" "bar")
		)
		"1"
		("+" "1" "1")
	)
	(
		("lambda" ("x") "x") "1"
	)
}

// INDEX
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
	index_expression,
	r#"
		{a[0]}[0]
		
		(list 1 2)[1]

		{ foo(,) }[0]
	"#,
	(
		"car" ("car" "a")
	)
	(
		"car" ("cdr" ("list" "1" "2"))
	)
	(
		"car" ("foo")
	)
}

// COND
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

// INFIX
test! {
	infix,
	r#"
		6 * 7
		1 + 2 - 3 / 2
		a : b : c
		a : (cons b c)
		{1 + 2} * 3
		(define
			(foo x y)
			{x + y}
		)
	"#,
	("*" "6" "7")
	("-" ("+" "1" "2") ("/" "3" "2"))
	("cons" "a" ("cons" "b" "c"))
	("cons" "a" ("cons" "b" "c"))
	("*" ("+" "1" "2") "3")
	("define"
		("foo" "x" "y")
		("+" "x" "y")
	)
}

// ESCAPING/MIXING
test! {
	mixed,
	r#"
		(+ 1 a(b,))
		(list a[0] a[1])
		(+ 1 { 6 * 7 })
		(foo cond { a => b, c => d })
	"#,
	("+" "1" ("a" "b"))
	("list"
		("car" "a")
		("car" ("cdr" "a"))
	)
	("+" "1" ("*" "6" "7"))
	("foo"
		("cond"
			("a" "b")
			("c" "d")
		)
	)
}

// LIST
test! {
	lists,
	r#"
		[1,2,3]
		[1,2,3,]
		[]
		( foo [1,2] )
		( foo [1,2,] )
		( foo [1,] )    ; trailing coma disambiguates list literal
		( foo [1] )     ; indexing into foo has precedence
		( foo [] )      ; with no argument list wins again
		{[1,2,3]}[1]    ; indexing possible when wrapped in {}
	"#,
	("list" "1" "2" "3")
	("list" "1" "2" "3")
	("list")
	("foo" ("list" "1" "2"))
	("foo" ("list" "1" "2"))
	("foo" ("list" "1"))
	(("car" ("cdr" "foo")))
	("foo" ("list"))
	("car" ("cdr" ("list" "1" "2" "3")))
}
