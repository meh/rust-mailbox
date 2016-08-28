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

use std::io::{self, Read};
use stream::{self, Entry};
use super::{Mail, Headers, Header};

pub struct Iter<R: Read> {
	input: stream::Iter<R>,
}

impl<R: Read> Iter<R> {
	pub fn new(input: R) -> Self {
		Iter {
			input: stream::read(input),
		}
	}
}

impl<R: Read> Iterator for Iter<R> {
	type Item = io::Result<Mail>;

	fn next(&mut self) -> Option<Self::Item> {
		macro_rules! eof {
			($body:expr) => (
				if let Some(value) = $body {
					value
				}
				else {
					return None;
				}
			);
		}

		macro_rules! try {
			($body:expr) => (
				match $body {
					Ok(value) =>
						value,

					Err(err) =>
						return Some(Err(err.into()))
				}
			);
		}

		let origin = if let Entry::Begin(origin) = try!(eof!(self.input.next())) {
			origin
		}
		else {
			return Some(Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid state")));
		};

		let mut headers = Headers::new();
		let mut content = Vec::new();
		let mut ended   = false;

		// Read headers.
		loop {
			match try!(eof!(self.input.next())) {
				Entry::Begin(_) => {
					return Some(Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid state")));
				}

				Entry::Escape(_) => (),

				// TODO(meh): handle multiple headers with same name
				Entry::Header(header) => {
					if let Ok(value) = Header::parse(header.key(), header.value()) {
						headers.insert(header.key().into(), value);
					}
					else {
						headers.insert(header.key().into(), Header::Value(header.value().into()));
					}
				}

				Entry::Content(value) => {
					content.push(value);
					break;
				}

				Entry::End => {
					ended = true;
					break;
				}
			}
		}

		// Read content unless there is none.
		if !ended {
			while let Entry::Content(value) = try!(eof!(self.input.next())) {
				content.push(value);
			}
		}

		Some(Ok(Mail::new(origin, headers, content)))
	}
}
