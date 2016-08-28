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
use super::Headers;

#[derive(Clone, Debug)]
pub struct Mail {
	origin:  stream::entry::Begin,
	headers: Headers,
	body:    Vec<Vec<u8>>,
}

impl Mail {
	pub fn new(origin: stream::entry::Begin, headers: Headers, body: Vec<Vec<u8>>) -> Self {
		Mail {
			origin:  origin,
			headers: headers,
			body:    body,
		}
	}

	pub fn origin(&self) -> &stream::entry::Begin {
		&self.origin
	}

	pub fn headers(&self) -> &Headers {
		&self.headers
	}

	pub fn body(&self) -> &Vec<Vec<u8>> {
		&self.body
	}
}
