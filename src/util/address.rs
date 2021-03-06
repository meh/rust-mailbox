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
use nom::IResult;
use stream::entry::header;

/// Represents an email address, composed of name, user and host.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Address {
	inner: header::Item,

	name: Option<Range<usize>>,
	user: Range<usize>,
	host: Option<Range<usize>>,
}

impl Address {
	pub(crate) fn ranges<T: AsRef<str>>(string: T) -> io::Result<(Option<Range<usize>>, Range<usize>, Option<Range<usize>>)> {
		let string = string.as_ref();

		if let IResult::Done(_, (name, user, host)) = parser::parse(string.as_bytes()) {
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

	#[inline]
	pub fn parse<T: AsRef<str>>(string: T) -> io::Result<Self> {
		Address::new(header::item(string.as_ref()))
	}

	#[inline]
	pub(crate) fn new(string: header::Item) -> io::Result<Self> {
		let (name, user, host) = try!(Address::ranges(&string));

		Ok(Address {
			inner: string,

			name: name,
			user: user,
			host: host,
		})
	}

	/// The name if any.
	///
	/// This is the first part of an address, which can be bare or quoted, for
	/// example `"Someone Somewhere" <someone@somewhe.re>` or `Someone Somewhere
	/// <someone@somewhe.re>`.
	#[inline]
	pub fn name(&self) -> Option<&str> {
		self.name.as_ref().map(|r| &self.inner[Range { start: r.start, end: r.end }])
	}

	/// The user.
	///
	/// This is the only mandatory part of an address, for instance it can be
	/// preceded by the `name` and followed by a `@` and the host, or be the
	/// only part of an address.
	#[inline]
	pub fn user(&self) -> &str {
		&self.inner[Range { start: self.user.start, end: self.user.end }]
	}

	/// The host if any.
	///
	/// This is the part after the `user` preceded by a `@`.
	#[inline]
	pub fn host(&self) -> Option<&str> {
		self.host.as_ref().map(|r| &self.inner[Range { start: r.start, end: r.end }])
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

mod parser {
	use std::str;
	use util::parser::{WSP, is_ws};

	named!(pub parse(&[u8]) -> (Option<&str>, &str, Option<&str>),
		do_parse!(
			take_while!(is_ws) >>
			name: opt!(complete!(name)) >>
			take_while!(is_ws) >>
			addr: address >>
			eof!() >>

			(unsafe {
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
			})));

	named!(name(&[u8]) -> &[u8],
		alt!(name_quoted | name_bare));

	named!(name_quoted(&[u8]) -> &[u8],
		do_parse!(
			name: delimited!(char!('"'), is_not!("\""), char!('"')) >>
			take_until!("<") >>

			(name)));

	named!(name_bare(&[u8]) -> &[u8],
		do_parse!(
			take_while!(is_ws) >>
			name: take_until!("<") >>

			(name)));

	named!(address(&[u8]) -> (&[u8], Option<&[u8]>),
		alt!(address_quoted | address_bare | address_user_only));

	named!(address_quoted(&[u8]) -> (&[u8], Option<&[u8]>),
		do_parse!(
			char!('<') >>
			user: take_until!("@") >>
			char!('@') >>
			host: take_until!(">") >>
			char!('>') >>

			(user, Some(host))));

	named!(address_bare(&[u8]) -> (&[u8], Option<&[u8]>),
		do_parse!(
			user: take_until!("@") >>
			char!('@') >>
			host: take_until_either_or_eof!(WSP) >>

			(user, Some(host))));

	named!(address_user_only(&[u8]) -> (&[u8], Option<&[u8]>),
		map!(take_until_either_or_eof!(WSP),
			|user| (user, None)));
}

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
