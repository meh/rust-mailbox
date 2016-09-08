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

pub trait Header: Sized {
	fn name() -> &'static str;
	fn parse(entries: &[entry::Header]) -> io::Result<Self>;
}

pub mod status;
pub use self::status::Status;

mod from;
pub use self::from::From;

mod x_envelope_from;
pub use self::x_envelope_from::XEnvelopeFrom;

mod to;
pub use self::to::To;

mod reply_to;
pub use self::reply_to::ReplyTo;

mod delivered_to;
pub use self::delivered_to::DeliveredTo;

mod return_path;
pub use self::return_path::ReturnPath;

mod date;
pub use self::date::Date;

mod x_remote_addr;
pub use self::x_remote_addr::XRemoteAddr;

mod content_length;
pub use self::content_length::ContentLength;

mod content_type;
pub use self::content_type::ContentType;

mod lines;
pub use self::lines::Lines;
