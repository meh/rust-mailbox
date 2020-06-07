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
use crate::util::Address;
use std::io;

pub struct MessageId(pub Address);

impl Header for MessageId {
    #[inline(always)]
    fn name() -> &'static str {
        "Message-ID"
    }

    #[inline]
    fn parse(values: &[header::Item]) -> io::Result<Self> {
        let address = r#try!(Address::new(values[0].clone()));

        if address.host().is_none() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "missing host"));
        }

        Ok(MessageId(address))
    }
}

impl MessageId {
    #[inline]
    pub fn id(&self) -> &str {
        self.0.user()
    }

    #[inline]
    pub fn host(&self) -> &str {
        self.0.host().unwrap()
    }
}
