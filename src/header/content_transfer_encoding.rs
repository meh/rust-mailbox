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
use stream::entry::header;
use super::Header;
use casing::Casing;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum ContentTransferEncoding {
	Ascii,
	ExtendedAscii,
	Binary,
	QuotedPrintable,
	Base64,
	Token(String),
}

impl Header for ContentTransferEncoding {
	#[inline]
	fn name() -> &'static str {
		"Content-Transfer-Encoding"
	}

	#[inline]
	fn parse(values: &[header::Item]) -> io::Result<Self> {
		Ok(match values[0].lower(Default::default()).as_ref() {
			"7bit"             => ContentTransferEncoding::Ascii,
			"8bit"             => ContentTransferEncoding::ExtendedAscii,
			"binary"           => ContentTransferEncoding::Binary,
			"quoted-printable" => ContentTransferEncoding::QuotedPrintable,
			"base64"           => ContentTransferEncoding::Base64,
			token              => ContentTransferEncoding::Token(token.into()),
		})
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use stream::entry::header;
	use header::Header;

	macro_rules! parse {
		($str:expr) => (
			<ContentTransferEncoding as Header>::parse(&[header::item($str)])
		);
	}

	#[test]
	fn insensitive() {
		assert_eq!(parse!("7Bit").unwrap(), ContentTransferEncoding::Ascii);
		assert_eq!(parse!("bAsE64").unwrap(), ContentTransferEncoding::Base64);
	}

	#[test]
	fn ascii() {
		assert_eq!(parse!("7BiT").unwrap(), ContentTransferEncoding::Ascii);
		assert_eq!(parse!("7bit").unwrap(), ContentTransferEncoding::Ascii);
	}

	#[test]
	fn extended_ascii() {
		assert_eq!(parse!("8BiT").unwrap(), ContentTransferEncoding::ExtendedAscii);
		assert_eq!(parse!("8bit").unwrap(), ContentTransferEncoding::ExtendedAscii);
	}
}
