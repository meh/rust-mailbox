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

#[derive(Clone, Debug)]
pub enum Entry {
	Begin(Begin),
	Escape(String),
	Header(Header),
	Body(Vec<u8>),
	End,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Begin {
	inner: String,

	address:   Range<usize>,
	timestamp: Range<usize>,
}

impl Begin {
	pub fn ranges<T: AsRef<str>>(string: T) -> io::Result<(Range<usize>, Range<usize>)> {
		let     string = string.as_ref();
		let mut parts  = string.splitn(3, ' ');

		if let Some("From") = parts.next() {
			if let Some(address) = parts.next() {
				let address = address.trim();

				if let Some(timestamp) = parts.next() {
					let timestamp = timestamp.trim();

					if timestamp.len() == 24 {
						let a = address.as_ptr() as usize - string.as_ptr() as usize;
						let t = timestamp.as_ptr() as usize - string.as_ptr() as usize;

						return Ok((
							Range { start: a, end: a + address.len() },
							Range { start: t, end: t + timestamp.len() },
						));
					}
				}
			}
		}

		Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid beginning"))
	}

	pub fn new(string: String) -> io::Result<Self> {
		let (address, timestamp) = try!(Begin::ranges(&string));

		Ok(Begin {
			inner: string,

			address:   address,
			timestamp: timestamp,
		})
	}

	pub fn into_inner(self) -> String {
		self.inner
	}

	pub fn address_range(&self) -> Range<usize> {
		Range { start: self.address.start, end: self.address.end }
	}

	pub fn address(&self) -> &str {
		&self.inner[self.address_range()]
	}

	pub fn timestamp_range(&self) -> Range<usize> {
		Range { start: self.timestamp.start, end: self.timestamp.end }
	}

	pub fn timestamp(&self) -> &str {
		&self.inner[self.timestamp_range()]
	}
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Header {
	inner: String,

	key:   Range<usize>,
	value: Range<usize>,
}

impl Header {
	pub fn ranges<T: AsRef<str>>(string: T) -> io::Result<(Range<usize>, Range<usize>)> {
		let     string = string.as_ref();
		let mut parts  = string.splitn(2, ':');

		if let Some(key) = parts.next() {
			let key = key.trim();

			if let Some(value) = parts.next() {
				let value = value.trim();

				{
					let k = key.as_ptr() as usize - string.as_ptr() as usize;
					let v = value.as_ptr() as usize - string.as_ptr() as usize;

					return Ok((
						Range { start: k, end: k + key.len() },
						Range { start: v, end: v + value.len() },
					));
				}
			}
		}

		Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid header"))
	}

	pub fn new(string: String) -> io::Result<Self> {
		let (key, value) = try!(Header::ranges(&string));

		Ok(Header {
			inner: string,

			key:   key,
			value: value,
		})
	}

	pub fn into_inner(self) -> String {
		self.inner
	}

	pub fn key_range(&self) -> Range<usize> {
		Range { start: self.key.start, end: self.key.end }
	}

	pub fn key(&self) -> &str {
		&self.inner[self.key_range()]
	}

	pub fn value_range(&self) -> Range<usize> {
		Range { start: self.value.start, end: self.value.end }
	}

	pub fn value(&self) -> &str {
		&self.inner[self.value_range()]
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn begin_ok() {
		let v = Begin::new("From foo@example.com Wed Nov 17 14:35:53 2010".into()).unwrap();
		assert_eq!(v.address(), "foo@example.com");
		assert_eq!(v.timestamp(), "Wed Nov 17 14:35:53 2010");
	}

	#[test]
	fn begin_fail() {
		assert!(Begin::new("From foo@example.com".into()).is_err());
	}

	#[test]
	fn header_ok() {
		let v = Header::new("From: meh. <meh@schizofreni.co>".into()).unwrap();
		assert_eq!(v.key(), "From");
		assert_eq!(v.value(), "meh. <meh@schizofreni.co>");
	}

	#[test]
	fn header_fail() {
		assert!(Header::new("From foo@example.com".into()).is_err());
	}
}
