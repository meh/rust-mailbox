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
use super::Header;

pub struct From(Address);

impl Header for From {
	#[inline(always)]
	fn name() -> &'static str {
		"From"
	}

	#[inline]
	fn parse(values: &[header::Item]) -> io::Result<Self> {
		Ok(From(try!(Address::new(values[0].clone()))))
	}
}

impl Deref for From {
	type Target = Address;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
