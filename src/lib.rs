//! This crate provides two simple wrappers
//! [`Mown`]
//! and
//! [`MownMut`]
//! for values that can be either owned or borrowed.
//! The type `Mown` is an simple `enum` type with two constructors:
//!
//! ```rust
//! # use std::borrow::Borrow;
//! pub trait ToOwned {
//! 	type Owned: Borrow<Self>;
//! }
//!
//! pub enum Mown<'a, T: ToOwned> {
//! 	Owned(T::Owned),
//! 	Borrowed(&'a T)
//! }
//! ```
//!
//! The mutable version `MownMut` follows the same definition with a mutable
//! reference.
//! This is very similar to the standard
//! [`Cow`](https://doc.rust-lang.org/std/borrow/enum.Cow.html)
//! type, except that it is not possible to transform a borrowed value into an owned
//! one.
//! This is also slightly different from the similar crate
//! [`boow`](https://crates.io/crates/boow)
//! since the [`ToOwned`] trait allow for the use of `Mown` with unsized types
//! (for instance `Mown<str>`) and with mutable references.
//!
//! ## Basic Usage
//!
//! One basic use case for the `Mown` type is the situation where one wants to
//! reuse some input borrowed value under some condition, or then use a custom
//! owned value.
//!
//! ```rust
//! use mown::Mown;
//!
//! fn function(input_value: &String) -> Mown<String> {
//! 	# let condition = true;
//! 	if condition {
//! 		Mown::Borrowed(input_value)
//! 	} else {
//! 		let custom_value: String = "foo_".to_string() + input_value + "_bar";
//! 		Mown::Owned(custom_value)
//! 	}
//! }
//! ```
//!
//! One can also wrap unsized types for which the provided [`ToOwned`]
//! trait has been implemented.
//! This is the case for the unsized `str` type with the sized owned type `String`.
//!
//! ```rust
//! use mown::Mown;
//!
//! fn function(input_value: &str) -> Mown<str> {
//! 	# let condition = true;
//! 	if condition {
//! 		Mown::Borrowed(input_value)
//! 	} else {
//! 		let custom_value: String = "foo_".to_string() + input_value + "_bar";
//! 		Mown::Owned(custom_value)
//! 	}
//! }
//! ```

use std::ops::{Deref, DerefMut};
use std::cmp::{PartialOrd, Ord, Ordering};
use std::hash::{Hash, Hasher};
use std::fmt::{self, Display, Debug, Formatter};
use std::borrow::{Borrow, BorrowMut};

/// Types that can be owned.
pub trait ToOwned {
	type Owned: Borrow<Self>;
}

impl<T: Sized> ToOwned for T {
	type Owned = T;
}

impl ToOwned for str {
	type Owned = String;
}

impl<T> ToOwned for [T] {
	type Owned = Vec<T>;
}

/// Container for borrowed or owned value.
pub enum Mown<'a, T: ?Sized + ToOwned> {
	/// Owned value.
	Owned(T::Owned),

	/// Borrowed value.
	Borrowed(&'a T)
}

impl<'a, T: ?Sized + ToOwned> Mown<'a, T> {
	/// Checks if the value is owned.
	pub fn is_owned(&self) -> bool {
		match self {
			Mown::Owned(_) => true,
			Mown::Borrowed(_) => false
		}
	}

	/// Checks if the value is borrowed.
	pub fn is_borrowed(&self) -> bool {
		match self {
			Mown::Owned(_) => false,
			Mown::Borrowed(_) => true
		}
	}

	/// Returns the owned value as a mutable reference, if any.
	///
	/// If the value is borrowed, returns `None`.
	pub fn as_mut(&mut self) -> Option<&mut T> where T::Owned: BorrowMut<T> {
		match self {
			Mown::Owned(t) => Some(t.borrow_mut()),
			Mown::Borrowed(_) => None
		}
	}
}

impl<'a, T: ?Sized + ToOwned> AsRef<T> for Mown<'a, T> {
	fn as_ref(&self) -> &T {
		match self {
			Mown::Owned(t) => t.borrow(),
			Mown::Borrowed(t) => t
		}
	}
}

impl<'a, T: ?Sized + ToOwned> Deref for Mown<'a, T> {
	type Target = T;

	fn deref(&self) -> &T {
		self.as_ref()
	}
}

impl<'a, T: ?Sized + ToOwned + PartialEq> PartialEq for Mown<'a, T> {
	fn eq(&self, other: &Mown<'a, T>) -> bool {
		self.as_ref() == other.as_ref()
	}
}

impl<'a, T: ?Sized + ToOwned + Eq> Eq for Mown<'a, T> { }

impl<'a, T: ?Sized + ToOwned + PartialOrd> PartialOrd for Mown<'a, T> {
	fn partial_cmp(&self, other: &Mown<'a, T>) -> Option<Ordering> {
		self.as_ref().partial_cmp(other)
	}
}

impl<'a, T: ?Sized + ToOwned + Ord> Ord for Mown<'a, T> {
	fn cmp(&self, other: &Mown<'a, T>) -> Ordering {
		self.as_ref().cmp(other)
	}
}

impl<'a, T: ?Sized + ToOwned + Hash> Hash for Mown<'a, T> {
	fn hash<H: Hasher>(&self, hasher: &mut H) {
		self.as_ref().hash(hasher)
	}
}

impl<'a, T: ?Sized + ToOwned + Display> Display for Mown<'a, T> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		self.as_ref().fmt(f)
	}
}

impl<'a, T: ?Sized + ToOwned + Debug> Debug for Mown<'a, T> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		self.as_ref().fmt(f)
	}
}

/// Container for mutabily borrowed or owned values.
pub enum MownMut<'a, T: ?Sized + ToOwned> {
	/// Owned value.
	Owned(T::Owned),

	/// Borrowed value.
	Borrowed(&'a mut T)
}

impl<'a, T: ?Sized + ToOwned> MownMut<'a, T> {
	/// Checks if the value is owned.
	pub fn is_owned(&self) -> bool {
		match self {
			MownMut::Owned(_) => true,
			MownMut::Borrowed(_) => false
		}
	}

	/// Checks if the value is borrowed.
	pub fn is_borrowed(&self) -> bool {
		match self {
			MownMut::Owned(_) => false,
			MownMut::Borrowed(_) => true
		}
	}
}

impl<'a, T: ?Sized + ToOwned> AsRef<T> for MownMut<'a, T> {
	fn as_ref(&self) -> &T {
		match self {
			MownMut::Owned(t) => t.borrow(),
			MownMut::Borrowed(t) => t
		}
	}
}

impl<'a, T: ?Sized + ToOwned> AsMut<T> for MownMut<'a, T> where T::Owned: BorrowMut<T> {
	fn as_mut(&mut self) -> &mut T {
		match self {
			MownMut::Owned(t) => t.borrow_mut(),
			MownMut::Borrowed(t) => t
		}
	}
}

impl<'a, T: ?Sized + ToOwned> Deref for MownMut<'a, T> {
	type Target = T;

	fn deref(&self) -> &T {
		self.as_ref()
	}
}

impl<'a, T: ?Sized + ToOwned> DerefMut for MownMut<'a, T> where T::Owned: BorrowMut<T> {
	fn deref_mut(&mut self) -> &mut T {
		self.as_mut()
	}
}

impl<'a, T: ?Sized + ToOwned + PartialEq> PartialEq for MownMut<'a, T> {
	fn eq(&self, other: &MownMut<'a, T>) -> bool {
		self.as_ref() == other.as_ref()
	}
}

impl<'a, T: ?Sized + ToOwned + Eq> Eq for MownMut<'a, T> { }

impl<'a, T: ?Sized + ToOwned + PartialOrd> PartialOrd for MownMut<'a, T> {
	fn partial_cmp(&self, other: &MownMut<'a, T>) -> Option<Ordering> {
		self.as_ref().partial_cmp(other)
	}
}

impl<'a, T: ?Sized + ToOwned + Ord> Ord for MownMut<'a, T> {
	fn cmp(&self, other: &MownMut<'a, T>) -> Ordering {
		self.as_ref().cmp(other)
	}
}

impl<'a, T: ?Sized + ToOwned + Hash> Hash for MownMut<'a, T> {
	fn hash<H: Hasher>(&self, hasher: &mut H) {
		self.as_ref().hash(hasher)
	}
}

impl<'a, T: ?Sized + ToOwned + Display> Display for MownMut<'a, T> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		self.as_ref().fmt(f)
	}
}

impl<'a, T: ?Sized + ToOwned + Debug> Debug for MownMut<'a, T> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		self.as_ref().fmt(f)
	}
}
