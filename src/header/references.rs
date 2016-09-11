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

use std::io;
use std::ops::Deref;
use stream::entry::header;
use util::Address;
use super::{Header, MessageId};

pub struct References(Vec<MessageId>);

impl Header for References {
	#[inline(always)]
	fn name() -> &'static str {
		"References"
	}

	#[inline]
	fn parse(values: &[header::Item]) -> io::Result<Self> {
		let mut ids    = Vec::new();
		let     string = values[0].clone();

		for slice in string.split(',') {
			let start = slice.as_ptr() as usize - string.as_ptr() as usize;
			let end   = start + slice.len();

			ids.push(MessageId(try!(Address::new(string.clone().map(|s| &s[start..end])))));
		}

		Ok(References(ids))
	}
}

impl Deref for References {
	type Target = [MessageId];

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
