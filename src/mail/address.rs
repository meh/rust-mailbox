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

use std::str::FromStr;
use std::fmt::{self, Write};
use std::io;
use regex::Regex;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Address {
	name: Option<String>,
	user: String,
	host: String,
}

impl Address {
	pub fn name(&self) -> Option<&str> {
		self.name.as_ref().map(|v| v.as_ref())
	}

	pub fn user(&self) -> &str {
		&self.user
	}

	pub fn host(&self) -> &str {
		&self.host
	}
}

impl fmt::Display for Address {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		if let Some(name) = self.name.as_ref() {
			try!(f.write_char('"'));
			try!(f.write_str(name));
			try!(f.write_char('"'));
			try!(f.write_char(' '));
			try!(f.write_char('<'));
		}

		try!(f.write_str(&self.user));
		try!(f.write_char('@'));
		try!(f.write_str(&self.host));

		if let Some(_) = self.name.as_ref() {
			try!(f.write_char('>'));
		}

		Ok(())
	}
}

lazy_static! {
	static ref ADDRESS: Regex = Regex::new(r#"(?x:^
		( # Address with name.
			(
				("(?P<name_quoted>[^"]+?)") # Address with name in quotes.
				|
				(?P<name_bare>[^"]+?) # Address with bare name.
			)
			\s*
			<
				\s*
				(?P<user_quoted>[^@\s]+) # User name.
				@
				(?P<host_quoted>[^\s]+?) # Host name.
				\s*
			>
		)
		|
		( # Address without name.
			<?
			\s*
			(?P<user_bare>[^@\s]+) # User name.
			@
			(?P<host_bare>[^\s]+?) # Host name.
			\s*
			>?
		)
	$)"#).unwrap();
}

impl FromStr for Address {
	type Err = io::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if let Some(captures) = ADDRESS.captures(s) {
			Ok(Address {
				name: captures.name("name_quoted").or_else(|| captures.name("name_bare")).map(|v| v.into()),
				user: captures.name("user_quoted").or_else(|| captures.name("user_bare")).unwrap().into(),
				host: captures.name("host_quoted").or_else(|| captures.name("host_bare")).unwrap().into(),
			})
		}
		else {
			Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid address"))
		}
	}
}

#[cfg(test)]
mod test {
	use std::str::FromStr;
	use super::*;

	#[test]
	fn parse_name_bare() {
		let v = Address::from_str(r#"culone <culo@culetto>"#).unwrap();
		assert_eq!(v.name(), Some("culone"));
		assert_eq!(v.user(), "culo");
		assert_eq!(v.host(), "culetto");
	}

	#[test]
	fn parse_name_quoted() {
		let v = Address::from_str(r#""culone" <culo@culetto>"#).unwrap();
		assert_eq!(v.name(), Some("culone"));
		assert_eq!(v.user(), "culo");
		assert_eq!(v.host(), "culetto");
	}

	#[test]
	fn parse_no_name() {
		let v = Address::from_str(r#"culo@culetto"#).unwrap();
		assert_eq!(v.user(), "culo");
		assert_eq!(v.host(), "culetto");
	}

	#[test]
	fn fail() {
		assert!(Address::from_str("hue").is_err());
	}
}
