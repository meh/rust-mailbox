//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004
//
// Copyleft (ↄ) meh. <meh@schizofreni.co> | http://meh.schizofreni.co
//
// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.

use std::ops::Range;
use std::io;
use std::rc::Rc;
use std::borrow::Cow;
use owning_ref::OwningRef;
use nom::IResult;
use casing::Casing;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Header {
	inner: Item,

	key:   Range<usize>,
	value: Range<usize>,
}

pub type Item = OwningRef<Rc<String>, str>;

#[inline(always)]
pub fn item<T: Into<String>>(string: T) -> Item {
	OwningRef::new(Rc::new(string.into())).map(|s| s.as_ref())
}

impl Header {
	#[inline]
	pub fn ranges<T: AsRef<[u8]>>(string: T) -> io::Result<(Range<usize>, Range<usize>)> {
		let string = string.as_ref();

		if let IResult::Done(_, (key, value)) = parser::parse(string) {
			let k = key.as_ptr() as usize - string.as_ptr() as usize;
			let v = value.as_ptr() as usize - string.as_ptr() as usize;

			Ok((
				Range { start: k, end: k + key.len() },
				Range { start: v, end: v + value.len() },
			))
		}
		else {
			Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid header"))
		}
	}

	#[inline]
	pub fn new<T: Into<Vec<u8>>>(string: T) -> io::Result<Self> {
		let string       = string.into();
		let (key, value) = try!(Header::ranges(&string));

		Ok(Header {
			inner: item(unsafe { String::from_utf8_unchecked(string) }),

			key:   key,
			value: value,
		})
	}

	#[inline]
	pub fn key(&self) -> Item {
		match (&self.inner[Range { start: self.key.start, end: self.key.end }]).header(Default::default()) {
			Cow::Borrowed(_)   => self.inner.clone().map(|s| &s[Range { start: self.key.start, end: self.key.end }]),
			Cow::Owned(string) => OwningRef::new(Rc::new(string)).map(|s| s.as_ref()),
		}
	}

	#[inline]
	pub fn value(&self) -> Item {
		self.inner.clone().map(|s| &s[Range { start: self.value.start, end: self.value.end }])
	}
}

mod parser {
	use nom::eof;
	use util::parser::{is_ws, is_printable_no_colon, is_printable_or_ws};

	named!(pub parse(&[u8]) -> (&[u8], &[u8]),
		chain!(
			key: key ~
			char!(':') ~
			take_while!(is_ws) ~
			value: value ~
			eof,

			|| { (key, value) }));

	named!(key(&[u8]) -> &[u8],
		take_while!(is_printable_no_colon));

	named!(value(&[u8]) -> &[u8],
		take_while!(is_printable_or_ws));
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn ok() {
		let v = Header::new("From: meh. <meh@schizofreni.co>").unwrap();
		assert_eq!(&*v.key(), "From");
		assert_eq!(&*v.value(), "meh. <meh@schizofreni.co>");
	}

	#[test]
	fn fail() {
		assert!(Header::new("From foo@example.com").is_err());
	}
}
