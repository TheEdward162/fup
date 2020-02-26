# FUP - Fuck Un(losed Parens

FUP is a preprocessor for Scheme programming language. It is intended to help avoid the overuse of parens in Scheme in situations when even rainbow colorizers stop helping.

FUP uses [pest.rs](https://pest.rs/) to parse Scheme and augmented expression. Augmented expressions are then converted to Scheme and valid Scheme code is output.

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
```
becomes
```scheme
(car (cdr lst))

(cdr (cdr lst))

(list
	(car c)
	(car (cdr c))
)

()

(
	list
	(car (cdr e))
	(car (cdr (cdr e)))
)
```

## Planned features

* Infixing for binary operators
* Implicitly scoped let statements
* FUP expressions inside Scheme expressions
