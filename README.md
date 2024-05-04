lib-endian
======

This crate is the simplest library for storing a value while handling byte order
or for obtaining the byte order of the current target.

## Examples

- Get the `Endian` of current target by `Endian::NATIVE`.

```rust
use lib_endian::Endian;
const NE: Endian = Endian::NATIVE;
```

- Get a native `Endian` or a reversed one by `Endian::native_or_reversed(bool)`.

```rust
use lib_endian::Endian;

const REVERSE: bool = true;

const NE_REV: Endian = Endian::native_or_reversed(REVERSE);
assert_eq!(NE_REV, Endian::NATIVE.reverse());

const NE: Endian = Endian::native_or_reversed(!REVERSE);
assert_eq!(NE, Endian::NATIVE);
```

- Reverse a `Endian` by `.reverse()` or `!`:

```rust
use lib_endian::Endian;

const BE: Endian = Endian::Little.reverse();
let le = !Endian::Big;
assert_eq!(BE, !le);
```

- Compare two `Endian` by `.is(...)`, `==` or `!=`:

```rust
use lib_endian::Endian;

const BE: Endian = Endian::Big;

const EQ: bool = BE.is(Endian::Big);
const NEQ: bool = !Endian::Little.is(BE);
let eq = Endian::Big == BE;
let neq = BE != Endian::Little;
assert!(EQ & NEQ & eq & neq);
```

## Attention

You Can't Use `!`, `==` or `!=` in
[Constant Expressions](https://doc.rust-lang.org/reference/const_eval.html)
because they are implemented by traits,
which, at least in Rust version `1.78.0`, do not contain `const fn`.

So in
[Constant Expressions](https://doc.rust-lang.org/reference/const_eval.html),
use `v.reverse()` to replace `!v`,
`a.is(b)` to replace `a==b`,
`!a.is(b)` to replace `a!=b`.