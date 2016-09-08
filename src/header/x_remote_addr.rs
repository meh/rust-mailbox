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
use std::net::IpAddr;
use stream::entry;
use super::Header;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct XRemoteAddr(IpAddr);

impl Header for XRemoteAddr {
	#[inline]
	fn name() -> &'static str {
		"X-Remote-Addr"
	}

	#[inline]
	fn parse(entries: &[entry::Header]) -> io::Result<Self> {
		Ok(XRemoteAddr(try!(entries[0].value().parse().map_err(|_|
			io::Error::new(io::ErrorKind::InvalidInput, "invalid IP address")))))
	}
}

impl Deref for XRemoteAddr {
	type Target = IpAddr;

	#[inline]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
