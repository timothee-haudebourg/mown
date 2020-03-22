//! This crate provides two simple wrappers
//! [`Mown`]
//! and
//! [`MownMut`]
//! for values that can be either owned or borrowed.
//! The type `Mown` is an simple `enum` type with two constructors:
//!
//! ```rust
//! pub enum Mown<'a, T> {
//! 	Owned(T),
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
//!
//! ## Basic Usage
//!
//! One basic use case for the `Mown` type is the situation where one wants to
//! reuse some input borrowed value under some condition, or then use a custom
//! owned value.
//!
//! ```rust
//! use mown::Mown;
//! # type T = ();
//!
//! fn function(input_value: &T) {
//! 	# let condition = true;
//! 	let value = if condition {
//! 		Mown::Borrowed(input_value)
//! 	} else {
//! 		# let init_custom_value = ();
//! 		let custom_value: T = init_custom_value ;
//! 		Mown::Owned(custom_value)
//! 	};
//!
//! 	// do something with `value`.
//! }
//! ```

use std::ops::{Deref, DerefMut};
use std::cmp::{PartialOrd, Ord, Ordering};
use std::hash::{Hash, Hasher};
use std::fmt::{self, Display, Debug, Formatter};

/// Container for borrowed or owned value.
pub enum Mown<'a, T> {
	/// Owned value.
	Owned(T),

	/// Borrowed value.
	Borrowed(&'a T)
}

impl<'a, T> Mown<'a, T> {
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
	pub fn as_mut(&mut self) -> Option<&mut T> {
		match self {
			Mown::Owned(ref mut t) => Some(t),
			Mown::Borrowed(_) => None
		}
	}
}

impl<'a, T> AsRef<T> for Mown<'a, T> {
	fn as_ref(&self) -> &T {
		match self {
			Mown::Owned(ref t) => t,
			Mown::Borrowed(t) => t
		}
	}
}

impl<'a, T> Deref for Mown<'a, T> {
	type Target = T;

	fn deref(&self) -> &T {
		self.as_ref()
	}
}

impl<'a, T: PartialEq> PartialEq for Mown<'a, T> {
	fn eq(&self, other: &Mown<'a, T>) -> bool {
		self.as_ref() == other.as_ref()
	}
}

impl<'a, T: Eq> Eq for Mown<'a, T> { }

impl<'a, T: PartialOrd> PartialOrd for Mown<'a, T> {
	fn partial_cmp(&self, other: &Mown<'a, T>) -> Option<Ordering> {
		self.as_ref().partial_cmp(other)
	}
}

impl<'a, T: Ord> Ord for Mown<'a, T> {
	fn cmp(&self, other: &Mown<'a, T>) -> Ordering {
		self.as_ref().cmp(other)
	}
}

impl<'a, T: Hash> Hash for Mown<'a, T> {
	fn hash<H: Hasher>(&self, hasher: &mut H) {
		self.as_ref().hash(hasher)
	}
}

impl<'a, T: Display> Display for Mown<'a, T> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		self.as_ref().fmt(f)
	}
}

impl<'a, T: Debug> Debug for Mown<'a, T> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		self.as_ref().fmt(f)
	}
}

/// Container for mutabily borrowed or owned values.
pub enum MownMut<'a, T> {
	/// Owned value.
	Owned(T),

	/// Borrowed value.
	Borrowed(&'a mut T)
}

impl<'a, T> MownMut<'a, T> {
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

impl<'a, T> AsRef<T> for MownMut<'a, T> {
	fn as_ref(&self) -> &T {
		match self {
			MownMut::Owned(ref t) => t,
			MownMut::Borrowed(t) => t
		}
	}
}

impl<'a, T> AsMut<T> for MownMut<'a, T> {
	fn as_mut(&mut self) -> &mut T {
		match self {
			MownMut::Owned(ref mut t) => t,
			MownMut::Borrowed(t) => t
		}
	}
}

impl<'a, T> Deref for MownMut<'a, T> {
	type Target = T;

	fn deref(&self) -> &T {
		self.as_ref()
	}
}

impl<'a, T> DerefMut for MownMut<'a, T> {
	fn deref_mut(&mut self) -> &mut T {
		self.as_mut()
	}
}

impl<'a, T: PartialEq> PartialEq for MownMut<'a, T> {
	fn eq(&self, other: &MownMut<'a, T>) -> bool {
		self.as_ref() == other.as_ref()
	}
}

impl<'a, T: Eq> Eq for MownMut<'a, T> { }

impl<'a, T: PartialOrd> PartialOrd for MownMut<'a, T> {
	fn partial_cmp(&self, other: &MownMut<'a, T>) -> Option<Ordering> {
		self.as_ref().partial_cmp(other)
	}
}

impl<'a, T: Ord> Ord for MownMut<'a, T> {
	fn cmp(&self, other: &MownMut<'a, T>) -> Ordering {
		self.as_ref().cmp(other)
	}
}

impl<'a, T: Hash> Hash for MownMut<'a, T> {
	fn hash<H: Hasher>(&self, hasher: &mut H) {
		self.as_ref().hash(hasher)
	}
}

impl<'a, T: Display> Display for MownMut<'a, T> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		self.as_ref().fmt(f)
	}
}

impl<'a, T: Debug> Debug for MownMut<'a, T> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		self.as_ref().fmt(f)
	}
}
