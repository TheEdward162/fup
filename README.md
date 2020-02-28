# FUP - Fuck Un(losed Parens

FUP is a preprocessor for Scheme programming language. It is intended to help avoid the overuse of parens in Scheme in situations when even rainbow colorizers stop helping.

FUP uses [pest.rs](https://pest.rs/) to parse Scheme and augmented expression. Augmented expressions are then converted to Scheme and valid Scheme code is output.

## Usage

The binary currently has two modes - file and repl.

To start the repl, run the binary with no arguments. To read input from file, start the binary with the first cli argument set to the file path.

Currenty, repl has the following meta commands:

Command | Description
--- | ---
`,exit` | Exits the repl
`,help` | Prints help with these commands
`,mline` | Toggles multiline and immediate evaluation

_**Note**: By default, repl runs in multiline mode, which means input is buffered until an empty newline is passed._

## FUP expressions

Currently, FUP supports these augmented expressions:

### Define

Helps the readability of define statements:

```scheme
define foo(x, y) {
	(display (+ x y))
}

define bar = z;
```
becomes
```scheme
(define
	(foo x y)
	(display (+ x y))
)

(define bar z)
```

### Function call

Functions are called in the same way as they are defined:

```scheme
foo(x, y)
```
becomes
```scheme
(foo x y)
```

However, calling functions with one or zero arguments has a caveat to avoid ambiguity with natural scheme code:

```scheme
foo (x) ; not a call

foo (x,) ; unambiguous call

foo(,) ; unambiguous call, no arguments

(lambda (x) x)(1,) ; scheme expression call

{ cond {
	#f => foo,
	#t => bar
} }(1,) ; fup expression call
```
becomes
```scheme
foo (x) ; not a call

(foo x) ; unambiguous call

(foo) ; unambiguous call, no arguments

((lambda (x) x) 1) ; scheme expression call

(
	(cond
		(#f foo)
		(#t bar)
	) 1
) ; fup expression call
```

### Cond

Cond works similar to `switch` statemets or the Rust `match` statement:

```scheme
cond {
	a => b,
	c => d,
	x => y,
	else => z
}
```
becomes
```scheme
(cond
	(a b)
	(c d)
	(x y)
	(else z)
)
```

### List indexing

Lists can be indexed to avoid nested `car`s and `cdr`s from making list accesses unreadable:

```scheme
lst[1] ; one element index

lst[2..] ; slice-after index

lst[..2] ; slice before index

lst[1..1] ; slice between index, empty

lst[1..3] ; slice between index

(list 1 2)[1] ; scheme expression index

{ foo(,) }[0] ; fup expression index
```
becomes
```scheme
(car (cdr lst)) ; one element index

(cdr (cdr lst)) ; slice-after index

(list
	(car lst)
	(car (cdr lst))
) ; slice before index

() ; slice between index, empty

(
	list
	(car (cdr lst))
	(car (cdr (cdr lst)))
) ; slice between index

(
	car (cdr (list 1 2))
) ; scheme expression index

(
	car (foo)
) ; fup expression index
```

_**Note**: Be aware that the `..N` and `N..M` variations evaluate the indexed expression as many times as the the length of the resulting list._

### List literal syntax

Lists can be created using `[item, item, ..]` syntax.

```scheme
[1,2,3]
[1,2,3,]
[]              ; useful as '()
( foo [1,2] )
( foo [1,2,] )
( foo [1,] )    ; trailing coma disambiguates list literal
( foo [1] )     ; indexing into foo has precedence
( foo [] )      ; with no argument list wins again
{[1,2,3]}[1]    ; indexing possible when wrapped in {}
```
becomes
```scheme
(list 1 2 3)
(list 1 2 3)
(list)          ; results in the same thing as '()
(foo (list 1 2))
(foo (list 1 2))
(foo (list 1))
((car (cdr foo)))
(foo (list))
(car (cdr (list 1 2 3)))
```

### Binary operator infixing

Binary operators can be infixed as in other langauges:
```scheme
1 + 2

a == b ; "==" is an alias for "equal?"

#t is #t ; "is" is an alias for "eq?"
```
becomes
```scheme
(+ 1 2)

(equal? a b)

(eq? #t #t)
```

_**Note**: To avoid ambiguity, infixed operators only work in non-scheme contexts._

### Escaping

Some FUP expressions (such as infixed operators) can be ambiguous in Scheme contexts. To explicitly create a FUP context and parse expressions as FUP expressions, use `{}` blocks:

```scheme
(define
	(foo x y)
	{x + y}
)
```
becomes
```scheme
(define
	(foo x y)
	(+ x y)
)
```

## Planned features

* Implicitly scoped let statements
