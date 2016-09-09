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
use nom::{eof, IResult};
use util::parser::{WS, is_whitespace};

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Begin {
	inner: String,

	address:   Range<usize>,
	timestamp: Range<usize>,
}

impl Begin {
	#[inline]
	pub fn ranges<T: AsRef<str>>(string: T) -> io::Result<(Range<usize>, Range<usize>)> {
		let string = string.as_ref().as_bytes();

		if let IResult::Done(_, (address, timestamp)) = parse(string) {
			let a = address.as_ptr() as usize - string.as_ptr() as usize;
			let t = timestamp.as_ptr() as usize - string.as_ptr() as usize;

			return Ok((
				Range { start: a, end: a + address.len() },
				Range { start: t, end: t + timestamp.len() },
			));
		}

		Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid beginning"))
	}

	#[inline]
	pub fn new<T: Into<String>>(string: T) -> io::Result<Self> {
		let string               = string.into();
		let (address, timestamp) = try!(Begin::ranges(&string));

		Ok(Begin {
			inner: string,

			address:   address,
			timestamp: timestamp,
		})
	}

	#[inline]
	pub fn address_range(&self) -> Range<usize> {
		Range { start: self.address.start, end: self.address.end }
	}

	#[inline]
	pub fn address(&self) -> &str {
		&self.inner[self.address_range()]
	}

	#[inline]
	pub fn timestamp_range(&self) -> Range<usize> {
		Range { start: self.timestamp.start, end: self.timestamp.end }
	}

	#[inline]
	pub fn timestamp(&self) -> &str {
		&self.inner[self.timestamp_range()]
	}
}

named!(parse(&[u8]) -> (&[u8], &[u8]),
	chain!(
		tag!("From ") ~
		take_while!(is_whitespace) ~
		addr: address ~
		take_while!(is_whitespace) ~
		time: timestamp ~
		eof,

		|| { (addr, time) }));

named!(address(&[u8]) -> &[u8],
	take_until_either!(WS));

named!(timestamp(&[u8]) -> &[u8],
	take!(24));

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn ok() {
		let v = Begin::new("From foo@example.com Wed Nov 17 14:35:53 2010".into()).unwrap();
		assert_eq!(v.address(), "foo@example.com");
		assert_eq!(v.timestamp(), "Wed Nov 17 14:35:53 2010");
	}

	#[test]
	fn fail() {
		assert!(Begin::new("From foo@example.com".into()).is_err());
	}
}
