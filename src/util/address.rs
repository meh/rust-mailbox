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

use std::fmt::{self, Write};
use std::ops::Range;
use std::io;
use std::str;
use nom::{eof, rest, IResult};
use util::parser::{is_whitespace};
use stream::entry::header;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Address {
	inner: header::Item,

	name: Option<Range<usize>>,
	user: Range<usize>,
	host: Option<Range<usize>>,
}

impl Address {
	pub fn ranges<T: AsRef<str>>(string: T) -> io::Result<(Option<Range<usize>>, Range<usize>, Option<Range<usize>>)> {
		let string = string.as_ref();

		if let IResult::Done(_, (name, user, host)) = parse(string.as_bytes()) {
			let n = name.map(|n| n.as_ptr() as usize - string.as_ptr() as usize);
			let u = user.as_ptr() as usize - string.as_ptr() as usize;
			let h = host.map(|h| h.as_ptr() as usize - string.as_ptr() as usize);

			Ok((
				n.map(|n| Range { start: n, end: n + name.unwrap().len() }),
				Range { start: u, end: u + user.len() },
				h.map(|h| Range { start: h, end: h + host.unwrap().len() }),
			))
		}
		else {
			Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid address"))
		}
	}

	pub fn parse<T: AsRef<str>>(string: T) -> io::Result<Self> {
		Address::new(header::item(string.as_ref()))
	}

	pub fn new(string: header::Item) -> io::Result<Self> {
		let (name, user, host) = try!(Address::ranges(&string));

		Ok(Address {
			inner: string,

			name: name,
			user: user,
			host: host,
		})
	}

	#[inline]
	pub fn name_range(&self) -> Option<Range<usize>>  {
		self.name.as_ref().map(|r| Range { start: r.start, end: r.end })
	}

	#[inline]
	pub fn name(&self) -> Option<&str> {
		self.name_range().map(|r| &self.inner[r])
	}

	#[inline]
	pub fn user_range(&self) -> Range<usize>  {
		Range { start: self.user.start, end: self.user.end }
	}

	#[inline]
	pub fn user(&self) -> &str {
		&self.inner[self.user_range()]
	}

	#[inline]
	pub fn host_range(&self) -> Option<Range<usize>>  {
		self.host.as_ref().map(|r| Range { start: r.start, end: r.end })
	}

	#[inline]
	pub fn host(&self) -> Option<&str> {
		self.host_range().map(|r| &self.inner[r])
	}
}

impl fmt::Display for Address {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		if let Some(name) = self.name() {
			try!(f.write_char('"'));
			try!(f.write_str(name));
			try!(f.write_char('"'));
			try!(f.write_char(' '));
			try!(f.write_char('<'));
		}

		try!(f.write_str(&self.user()));

		if let Some(host) = self.host() {
			try!(f.write_char('@'));
			try!(f.write_str(host));
		}

		if self.name().is_some() {
			try!(f.write_char('>'));
		}

		Ok(())
	}
}

named!(parse(&[u8]) -> (Option<&str>, &str, Option<&str>),
	chain!(
		take_while!(is_whitespace) ~
		name: opt!(complete!(name)) ~
		take_while!(is_whitespace) ~
		addr: address ~
		eof,

		|| unsafe {
			let name = name.and_then(|s| {
				let value = str::from_utf8_unchecked(s).trim();

				if value.len() > 0 {
					Some(value)
				}
				else {
					None
				}
			});

			let user = str::from_utf8_unchecked(addr.0);
			let host = addr.1.map(|s| str::from_utf8_unchecked(s));

			(name, user, host)
		}));

named!(name(&[u8]) -> &[u8],
	alt!(name_quoted | name_bare));

named!(name_quoted(&[u8]) -> &[u8],
	chain!(
		name: delimited!(char!('"'), is_not!("\""), char!('"')) ~
		take_until!("<"),

		|| { name }));

named!(name_bare(&[u8]) -> &[u8],
	chain!(
		take_while!(is_whitespace) ~
		name: take_until!("<"),

		|| { name }));

named!(address(&[u8]) -> (&[u8], Option<&[u8]>),
	alt!(address_quoted | address_bare | address_user_only));

named!(address_quoted(&[u8]) -> (&[u8], Option<&[u8]>),
	chain!(
		char!('<') ~
		user: take_until!("@") ~
		char!('@') ~
		host: take_until!(">") ~
		char!('>'),

		|| { (user, Some(host)) }));

named!(address_bare(&[u8]) -> (&[u8], Option<&[u8]>),
	chain!(
		user: take_until!("@") ~
		char!('@') ~
		host: rest, // FIXME: actually need a take_until!(WS) | eof

		|| { (user, Some(host)) }));

named!(address_user_only(&[u8]) -> (&[u8], Option<&[u8]>),
	chain!(
		user: rest, // FIXME: actually need a take_until!(WS) | eof

		|| { (user, None) }));

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn parse_name_bare() {
		let v = Address::parse(r#"culone <culo@culetto>"#).unwrap();
		assert_eq!(v.name(), Some("culone"));
		assert_eq!(v.user(), "culo");
		assert_eq!(v.host(), Some("culetto"));
	}

	#[test]
	fn parse_name_quoted() {
		let v = Address::parse(r#""culone" <culo@culetto>"#).unwrap();
		assert_eq!(v.name(), Some("culone"));
		assert_eq!(v.user(), "culo");
		assert_eq!(v.host(), Some("culetto"));
	}

	#[test]
	fn parse_no_name() {
		let v = Address::parse(r#"culo@culetto"#).unwrap();
		assert_eq!(v.user(), "culo");
		assert_eq!(v.host(), Some("culetto"));
	}

	#[test]
	fn parse_no_name_quoted() {
		let v = Address::parse(r#"<culo@culetto>"#).unwrap();
		assert!(v.name().is_none());
		assert_eq!(v.user(), "culo");
		assert_eq!(v.host(), Some("culetto"));
	}

	#[test]
	fn parse_just_name() {
		let v = Address::parse(r#"culo"#).unwrap();
		assert_eq!(v.user(), "culo");
	}
}
