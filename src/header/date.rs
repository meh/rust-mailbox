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
use chrono::{DateTime, FixedOffset};
use std::io;
use std::ops::Deref;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
pub struct Date(DateTime<FixedOffset>);

impl Header for Date {
    #[inline(always)]
    fn name() -> &'static str {
        "Date"
    }

    #[inline]
    fn parse(values: &[header::Item]) -> io::Result<Self> {
        DateTime::parse_from_rfc2822(values[0].as_ref())
            .map(Date)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid date"))
    }
}

impl Deref for Date {
    type Target = DateTime<FixedOffset>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
