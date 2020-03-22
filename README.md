 # Maybe Owned values

<table><tr>
	<td><a href="https://docs.rs/mown">Documentation</a></td>
	<td><a href="https://crates.io/crates/mown">Crate informations</a></td>
	<td><a href="https://github.com/timothee-haudebourg/mown">Repository</a></td>
</tr></table>

This crate provides two simple wrappers
[`Mown`](https://docs.rs/mown/latest/mown/enum.Mown.html)
and
[`MownMut`](https://docs.rs/mown/latest/mown/enum.MownMut.html)
for values that can be either owned or borrowed.
The type `Mown` is an simple `enum` type with two constructors:

```rust
pub enum Mown<'a, T> {
	Owned(T),
	Borrowed(&'a T)
}
```

The mutable version `MownMut` follows the same definition with a mutable
reference.
This is very similar to the standard
[`Cow`](https://doc.rust-lang.org/std/borrow/enum.Cow.html)
type, except that it is not possible to transform a borrowed value into an owned
one.

## Basic Usage

One basic use case for the `Mown` type is the situation where one wants to
reuse some input borrowed value under some condition, or then use a custom
owned value.

```rust
use mown::Mown;

fn function(input_value: &T) {
	let value = if condition {
		Mown::Borrowed(input_value)
	} else {
		let custom_value: T = ... ;
		Mown::Owned(custom_value)
	};

	// do something with `value`.
}
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
