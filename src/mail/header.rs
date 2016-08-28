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
use std::str::FromStr;
use std::net::IpAddr;
use mime::Mime;
use super::{Status, Address, Date};

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Header {
	Status(Status),

	From(Address),
	To(Address),
	Cc(Vec<Address>),

	Date(Date),

	RemoteAddr(IpAddr),

	ContentLength(usize),
	ContentType(Mime),
	Lines(usize),
}

impl Header {
	pub fn parse<T: AsRef<str>, T2: AsRef<str>, T3: AsRef<[T2]>>(name: T, value: T3) -> io::Result<Self> {
		let name  = name.as_ref();
		let value = value.as_ref();

		Ok(match name {
			"From" | "X-Envelope-From" =>
				Header::From(try!(Address::from_str(value[0].as_ref()))),

			"To" | "Reply-To" | "Delivered-To" | "Return-Path" =>
				Header::To(try!(Address::from_str(value[0].as_ref()))),

			"Cc" =>
				Header::Cc(try!(value[0].as_ref().split(',').map(|v| Address::from_str(v)).collect())),

			"Date" =>
				Header::Date(try!(Date::from_str(value[0].as_ref()))),

			"Status" | "X-Status" =>
				Header::Status(try!(Status::from_str(value[0].as_ref()))),

			"X-Remote-Addr" =>
				Header::RemoteAddr(try!(IpAddr::from_str(value[0].as_ref()).map_err(|_|
					io::Error::new(io::ErrorKind::InvalidInput, "invalid IP address")))),

			"Content-Length" =>
				Header::ContentLength(try!(value[0].as_ref().parse().map_err(|_|
					io::Error::new(io::ErrorKind::InvalidInput, "invalid content length")))),

			"Content-Type" =>
				Header::ContentType(try!(Mime::from_str(value[0].as_ref()).map_err(|_|
					io::Error::new(io::ErrorKind::InvalidInput, "invalid MIME type")))),

			"Lines" =>
				Header::Lines(try!(value[0].as_ref().parse().map_err(|_|
					io::Error::new(io::ErrorKind::InvalidInput, "invalid content length")))),

			_ =>
				return Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid content length"))
		})
	}
}
