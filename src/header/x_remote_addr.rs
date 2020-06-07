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

use super::Header;
use crate::stream::entry::header;
use std::io;
use std::net::IpAddr;
use std::ops::Deref;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct XRemoteAddr(IpAddr);

impl Header for XRemoteAddr {
    #[inline(always)]
    fn name() -> &'static str {
        "X-Remote-Addr"
    }

    #[inline]
    fn parse(values: &[header::Item]) -> io::Result<Self> {
        Ok(XRemoteAddr(r#try!(values[0].parse().map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidInput, "invalid IP address")
        }))))
    }
}

impl Deref for XRemoteAddr {
    type Target = IpAddr;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
