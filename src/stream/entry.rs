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
use std::io;

#[derive(Clone, Debug)]
pub enum Entry {
	Begin(Begin),
	Escape(String),
	Header(Header),
	Content(Vec<u8>),
	End,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Begin {
	address:   String,
	timestamp: String,
}

impl Begin {
	pub fn address(&self) -> &str {
		&self.address
	}

	pub fn timestamp(&self) -> &str {
		&self.timestamp
	}
}

impl FromStr for Begin {
	type Err = io::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut parts = s.splitn(3, ' ');

		if let Some("From") = parts.next() {
			if let Some(address) = parts.next() {
				if let Some(timestamp) = parts.next() {
					if timestamp.trim().len() == 24 {
						return Ok(Begin {
							address:   address.trim().into(),
							timestamp: timestamp.trim().into(),
						});
					}
				}
			}
		}

		Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid beginning"))
	} 
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Header {
	key:   String,
	value: String,
}

impl Header {
	pub fn key(&self) -> &str {
		&self.key
	}

	pub fn value(&self) -> &str {
		&self.value
	}
}

impl FromStr for Header {
	type Err = io::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut parts = s.splitn(2, ':');

		if let Some(key) = parts.next() {
			if let Some(value) = parts.next() {
				return Ok(Header {
					key: key.trim().into(),
					value: value.trim().replace("\t", " "),
				});
			}
		}

		Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid header"))
	}
}

#[cfg(test)]
mod test {
	use std::str::FromStr;
	use super::*;

	#[test]
	fn begin_ok() {
		let v = Begin::from_str("From foo@example.com 2016-08-27 17:10:19").unwrap();
		assert_eq!(v.address(), "foo@example.com");
		assert_eq!(v.timestamp(), "2016-08-27 17:10:19");
	}

	#[test]
	fn begin_fail() {
		assert!(Begin::from_str("From foo@example.com").is_err());
	}

	#[test]
	fn header_ok() {
		let v = Header::from_str("From: meh. <meh@schizofreni.co>").unwrap();
		assert_eq!(v.key(), "From");
		assert_eq!(v.value(), "meh. <meh@schizofreni.co>");
	}

	#[test]
	fn header_fail() {
		assert!(Header::from_str("From foo@example.com").is_err());
	}
}
