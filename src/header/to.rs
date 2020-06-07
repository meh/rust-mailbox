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
use std::ops::Deref;

pub struct To(Address);

impl Header for To {
    #[inline(always)]
    fn name() -> &'static str {
        "To"
    }

    #[inline]
    fn parse(values: &[header::Item]) -> io::Result<Self> {
        Ok(To(r#try!(Address::new(values[0].clone()))))
    }
}

impl Deref for To {
    type Target = Address;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
