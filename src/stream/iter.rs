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

use std::io::{self, BufReader, Read};
use std::iter::Peekable;
use super::{entry, Entry, Lines};

/// `Iterator` over line based entries.
///
/// This is a lower level building block that exposes an event like interface
/// over the line based one.
///
/// It will repeat a sequence of `Entry`s in the following order:
///
/// - `Entry::Begin`, the beginning of an email.
/// - `Entry::Header`, 0 or more header entries.
/// - `Entry::Body`, 0 or more body entries.
/// - `Entry::End`, the end of the email.
///
/// This is then leveraged by `mail::Iter` to expose a more ergonomic API over
/// actual `Mail`s.
pub struct Iter<R: Read> {
	input: Peekable<Lines<BufReader<R>>>,
	state: State,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum State {
	Begin,
	Header,
	Body,
}

impl<R: Read> Iter<R> {
	/// Create a new `Iterator` from the given input.
	#[inline]
	pub fn new(input: R) -> Self {
		Iter {
			input: super::lines(input).peekable(),
			state: State::Begin,
		}
	}
}

impl<R: Read> Iterator for Iter<R> {
	type Item = io::Result<Entry>;

	fn next(&mut self) -> Option<Self::Item> {
		macro_rules! eof {
			($body:expr) => (
				if let Some(value) = $body {
					value
				}
				else {
					if self.state == State::Body {
						self.state = State::Begin;
						return Some(Ok(Entry::End));
					}

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

		macro_rules! utf8 {
			($body:expr) => (
				match $body {
					Ok(value) =>
						value,

					Err(_) =>
						return Some(Err(io::Error::new(io::ErrorKind::InvalidData, "stream did not contain valid UTF-8")))
				}
			);
		}

		loop {
			let (offset, line) = r#try!(eof!(self.input.next()));

			match self.state {
				State::Begin => {
					// Parse the beginning and return any errors.
					let value  = r#try!(entry::Begin::new(utf8!(String::from_utf8(line))));
					self.state = State::Header;

					return Some(Ok(Entry::Begin(offset, value)));
				}

				State::Header => {
					// If the line is empty the header section is over.
					if line.is_empty() {
						self.state = State::Body;
						continue;
					}

					// There's an escaped line after the beginning.
					if line[0] == b'>' {
						continue;
					}

					let mut line = line;

					// Read lines until there are no folded headers.
					loop {
						let consumed;

						if let Ok((_, ref current)) = *eof!(self.input.peek()) {
							match current.first() {
								Some(&b' ') | Some(&b'\t') => {
									line.extend_from_slice(current);
									consumed = true;
								}

								_ => break
							}
						}
						else {
							break;
						}

						if consumed {
							self.input.next();
						}
					}

					// Parse the header and return any errors.
					return Some(Ok(Entry::Header(r#try!(entry::Header::new(line)))));
				}

				State::Body => {
					// If the line is empty there's a newline in the content or a new
					// mail is beginning.
					if line.is_empty() {
						if let Ok((_, ref current)) = *eof!(self.input.peek()) {
							// Try to parse the beginning, if it parses it's a new mail.
							if entry::Begin::ranges(current).is_ok() {
								self.state = State::Begin;
								return Some(Ok(Entry::End));
							}
						}
					}

					return Some(Ok(Entry::Body(line)));
				}
			}
		}
	}
}

#[cfg(test)]
mod test {
	use std::io::Cursor;
	use super::*;
	use super::super::Entry;

	#[test]
	fn simple() {
		let mut iter = Iter::new(Cursor::new("From meh@schizofreni.co Wed Nov 17 14:35:53 2010\r\nSubject: I like trains\r\nFoo: bar\r\n baz\r\n\r\nHi!\r\n"));

		{
			if let Entry::Begin(_, item) = iter.next().unwrap().unwrap() {
				assert_eq!(item.address(), "meh@schizofreni.co");
				assert_eq!(item.timestamp(), "Wed Nov 17 14:35:53 2010");
			}
			else {
				assert!(false);
			}
		}

		{
			if let Entry::Header(item) = iter.next().unwrap().unwrap() {
				assert_eq!(&*item.key(), "Subject");
				assert_eq!(&*item.value(), "I like trains");
			}
			else {
				assert!(false);
			}
		}

		{
			if let Entry::Header(item) = iter.next().unwrap().unwrap() {
				assert_eq!(&*item.key(), "Foo");
				assert_eq!(&*item.value(), "bar baz");
			}
			else {
				assert!(false);
			}
		}
	}
}
