 # Maybe Owned values

<table><tr>
	<td><a href="https://docs.rs/mown">Documentation</a></td>
	<td><a href="https://crates.io/crates/mown">Crate informations</a></td>
	<td><a href="https://github.com/timothee-haudebourg/mown">Repository</a></td>
</tr></table>

<!-- cargo-rdme start -->

This crate provides two simple wrappers
[`Mown`](https://docs.rs/mown/latest/mown/enum.Mown.html)
and
[`MownMut`](https://docs.rs/mown/latest/mown/enum.MownMut.html)
for values that can be either owned or borrowed.
The type `Mown` is an simple `enum` type with two constructors:

```rust
pub trait Borrowed {
  type Owned: Borrow<Self>;
}

pub enum Mown<'a, T: Borrowed> {
  Owned(T::Owned),
  Borrowed(&'a T)
}
```

The mutable version `MownMut` follows the same definition with a mutable
reference.
This is very similar to the standard
[`Cow`](https://doc.rust-lang.org/std/borrow/enum.Cow.html)
type, except that it is not possible to transform a borrowed value into an owned
one.
This is also slightly different from the similar crate
[`boow`](https://crates.io/crates/boow)
since the [`ToOwned`] trait allow for the use of `Mown` with unsized types
(for instance `Mown<str>`) and with mutable references.

### Basic Usage

One basic use case for the `Mown` type is the situation where one wants to
reuse some input borrowed value under some condition, or then use a custom
owned value.

```rust
use mown::Mown;

fn function(input_value: &String) -> Mown<String> {
  if condition {
    Mown::Borrowed(input_value)
  } else {
    let custom_value: String = "foo_".to_string() + input_value + "_bar";
    Mown::Owned(custom_value)
  }
}
```

One can also wrap unsized types for which the provided [`ToOwned`]
trait has been implemented.
This is the case for the unsized `str` type with the sized owned type `String`.

```rust
use mown::Mown;

fn function(input_value: &str) -> Mown<str> {
  if condition {
    Mown::Borrowed(input_value)
  } else {
    let custom_value: String = "foo_".to_string() + input_value + "_bar";
    Mown::Owned(custom_value)
  }
}
```

<!-- cargo-rdme end -->

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
