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
use mime::Mime;
use stream::entry::header;
use super::Header;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ContentType(Mime);

impl Header for ContentType {
	#[inline(always)]
	fn name() -> &'static str {
		"Content-Length"
	}

	#[inline]
	fn parse(values: &[header::Item]) -> io::Result<Self> {
		Ok(ContentType(try!(values[0].parse().map_err(|_|
			io::Error::new(io::ErrorKind::InvalidInput, "invalid MIME type")))))
	}
}

impl Deref for ContentType {
	type Target = Mime;

	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
