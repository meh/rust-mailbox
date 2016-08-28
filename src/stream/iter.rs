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

use std::io::{self, BufRead, BufReader, Read};
use std::str::{self, FromStr};
use super::{entry, Entry};

pub struct Iter<R: Read> {
	input: Lines<BufReader<R>>,
	cache: Option<Vec<u8>>,
	state: State,
}

pub struct Lines<R: BufRead>(R);

impl<R: BufRead> Iterator for Lines<R> {
	type Item = io::Result<Vec<u8>>;

	fn next(&mut self) -> Option<Self::Item> {
		let mut buffer = Vec::new();

		match self.0.read_until(b'\n', &mut buffer) {
			Ok(0) => {
				None
			}

			Ok(_) => {
				if buffer.last() == Some(&b'\n') {
					buffer.pop();

					if buffer.last() == Some(&b'\r') {
						buffer.pop();
					}
				}

				Some(Ok(buffer))
			}

			Err(e) => {
				Some(Err(e))
			}
		}
	}
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum State {
	Begin,
	Header,
	Content,
}

impl<R: Read> Iter<R> {
	pub fn new(input: R) -> Self {
		Iter {
			input: Lines(BufReader::new(input)),
			cache: None,
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
					if self.state == State::Content {
						self.state = State::Begin;
						return Some(Ok(Entry::End));
					}

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
			// Fetch a new line or use the cached line.
			let line = if let Some(cache) = self.cache.take() {
				cache
			}
			else {
				try!(eof!(self.input.next()))
			};

			match self.state {
				State::Begin => {
					// Parse the beginning and return any errors.
					let value  = try!(entry::Begin::from_str(utf8!(str::from_utf8(&line))));
					self.state = State::Header;

					return Some(Ok(Entry::Begin(value)));
				}

				State::Header => {
					let mut line = utf8!(String::from_utf8(line));

					// If the line is empty the header section is over.
					if line.is_empty() {
						self.state = State::Content;
						continue;
					}

					// There's an escaped line after the beginning.
					if line.starts_with(">") {
						return Some(Ok(Entry::Escape((&line[1..]).into())));
					}

					// Read lines until there are no folded headers.
					loop {
						let current = try!(eof!(self.input.next()));

						if current.first() == Some(&b' ') || current.first() == Some(&b'\t') {
							line.push_str(utf8!(str::from_utf8(&current)));
						}
						else {
							self.cache = Some(current);
							break;
						}
					}

					// Parse the header and return any errors.
					return Some(Ok(Entry::Header(try!(entry::Header::from_str(&line)))));
				}

				State::Content => {
					if line.is_empty() {
						self.cache = Some(try!(eof!(self.input.next())));

						if self.cache.as_ref().unwrap().starts_with(b"From ") {
							if let Ok(string) = str::from_utf8(self.cache.as_ref().unwrap()) {
								if entry::Begin::from_str(string).is_ok() {
									self.state = State::Begin;
									return Some(Ok(Entry::End));
								}
							}
						}

						return Some(Ok(Entry::Content("".into())));
					}
					else {
						return Some(Ok(Entry::Content(line)));
					}
				}
			}

			return None;
		}
	}
}

#[cfg(test)]
mod test {
	use std::io::Cursor;
	use super::*;
	use super::super::{entry, Entry};

	#[test]
	fn simple() {
		let mut iter = Iter::new(Cursor::new("From meh@schizofreni.co Wed Nov 17 14:35:53 2010\r\nSubject: I like trains\r\nFoo: bar\r\n baz\r\n\r\nHi!\r\n"));

		{
			let item = iter.next();
			assert!(item.is_some());
			if let Entry::Begin(item) = item.unwrap().unwrap() {
				assert_eq!(item.address, "meh@schizofreni.co");
				assert_eq!(item.timestamp, "Wed Nov 17 14:35:53 2010");
			}
			else {
				assert!(false);
			}
		}

		{
			let item = iter.next();
			assert!(item.is_some());
			if let Entry::Header(item) = item.unwrap().unwrap() {
				assert_eq!(item.key, "Subject");
				assert_eq!(item.value, "I like trains");
			}
			else {
				assert!(false);
			}
		}

		{
			let item = iter.next();
			assert!(item.is_some());
			if let Entry::Header(item) = item.unwrap().unwrap() {
				assert_eq!(item.key, "Foo");
				assert_eq!(item.value, "bar baz");
			}
			else {
				assert!(false);
			}
		}
	}
}
