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

```
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

```
foo(x, y)
```
becomes
```scheme
(foo x y)
```

However, calling functions with one or zero arguments has a caveat to avoid ambiguity with natural scheme code:

```
foo (x)

foo (x,)

foo(,)

(lambda (x) x)(1,)

{ cond {
	#f => foo,
	#t => bar
} }(1,)
```
becomes
```scheme
foo (x)

(foo x)

(foo)

((lambda (x) x) 1)

(
	(cond
		(#f foo)
		(#t bar)
	) 1
)
```

### Cond

Cond works similar to `switch` statemets or the Rust `match` statement:

```
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

```
lst[1]

lst[2..]

lst[..2]

lst[1..1]

lst[1..3]

(list 1 2)[1]

{ foo(,) }[0]
```
becomes
```scheme
(car (cdr lst))

(cdr (cdr lst))

(list
	(car lst)
	(car (cdr lst))
)

()

(
	list
	(car (cdr lst))
	(car (cdr (cdr lst)))
)

(
	car (cdr (list 1 2))
)

(
	car (foo)
)
```

_**Note**: Be aware that the `..N` and `N..M` variations evaluate the indexed expression as many times as the the length of the resulting list._

### Binary operator infixing

Binary operators can be infixed as in other langauges:
```
1 + 2

a == b

#t is #t
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

```
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
