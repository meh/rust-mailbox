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

use stream;
use super::{Headers, Body};

#[derive(Clone, Debug)]
pub struct Mail {
	offset:  u64,
	origin:  stream::entry::Begin,
	headers: Headers,
	body:    Body,
}

impl Mail {
	#[inline]
	pub fn new(offset: u64, origin: stream::entry::Begin, headers: Headers, body: Body) -> Self {
		Mail {
			offset:  offset,
			origin:  origin,
			headers: headers,
			body:    body,
		}
	}

	#[inline]
	pub fn offset(&self) -> u64 {
		self.offset
	}

	#[inline]
	pub fn origin(&self) -> &stream::entry::Begin {
		&self.origin
	}

	#[inline]
	pub fn headers(&self) -> &Headers {
		&self.headers
	}

	#[inline]
	pub fn body(&self) -> &Body {
		&self.body
	}
}
