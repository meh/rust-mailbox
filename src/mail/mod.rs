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

mod mail;
pub use self::mail::Mail;

mod headers;
pub use self::headers::Headers;

mod header;
pub use self::header::Header;

mod body;
pub use self::body::Body;

pub mod status;
pub use self::status::Status;

mod date;
pub use self::date::Date;

mod iter;
pub use self::iter::Iter;

use std::io::Read;

#[inline]
pub fn read<R: Read>(input: R) -> Iter<R> {
	Iter::new(input)
}
