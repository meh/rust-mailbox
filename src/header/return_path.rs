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
use stream::entry;
use util::Address;
use super::Header;

pub struct ReturnPath {
	address: Address,
}

impl Header for ReturnPath {
	#[inline]
	fn name() -> &'static str {
		"Return-Path"
	}

	#[inline]
	fn parse(entries: &[entry::Header]) -> io::Result<Self> {
		Ok(ReturnPath {
			address: try!(Address::new(entries[0].value()))
		})
	}
}

impl ReturnPath {
	#[inline]
	pub fn address(&self) -> &Address {
		&self.address
	}
}
