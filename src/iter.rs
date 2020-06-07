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
use crate::stream::{self, Entry};
use crate::mail::{Mail, Headers, Body};

pub struct Iter<R: Read> {
	input: stream::Iter<R>,
	body:  bool,
}

impl<R: Read> Iter<R> {
	#[inline]
	pub fn new(input: R) -> Self {
		Iter {
			input: stream::entries(input),
			body:  true,
		}
	}

	#[inline]
	pub fn body(&mut self, value: bool) -> &mut Self {
		self.body = value;
		self
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

		macro_rules! r#try {
			($body:expr) => (
				match $body {
					Ok(value) =>
						value,

					Err(err) =>
						return Some(Err(err.into()))
				}
			);
		}

		// The first entry must be an `Entry::Begin`.
		let (offset, origin) = if let Entry::Begin(offset, origin) = r#try!(eof!(self.input.next())) {
			(offset, origin)
		}
		else {
			return Some(Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid state")));
		};

		let mut headers = Headers::default();
		let mut body    = Body::default();
		let mut ended   = false;

		// Read headers.
		loop {
			match r#try!(eof!(self.input.next())) {
				// This shouldn't happen.
				Entry::Begin(..) => {
					return Some(Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid state")));
				}

				// Insert the header.
				Entry::Header(header) => {
					headers.insert(header);
				}

				// The body started.
				Entry::Body(value) => {
					if self.body {
						body.append(value);
					}

					break;
				}

				// There was no body.
				Entry::End => {
					ended = true;
					break;
				}
			}
		}

		// Read body if there is one.
		if !ended {
			while let Entry::Body(value) = r#try!(eof!(self.input.next())) {
				if self.body {
					body.append(value);
				}
			}
		}

		Some(Ok(Mail::new(offset, origin, headers, body)))
	}
}
