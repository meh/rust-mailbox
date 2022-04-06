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
use std::ops::Range;
use std::str;

/// The beginning of a new email.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Begin {
    inner: String,

    address: Range<usize>,
    timestamp: Range<usize>,
}

impl Begin {
    #[inline]
    pub(crate) fn ranges<T: AsRef<[u8]>>(string: T) -> io::Result<(Range<usize>, Range<usize>)> {
        let string = string.as_ref();

        if let Ok((_, (address, timestamp))) = parser::parse(string) {
            let a = address.as_ptr() as usize - string.as_ptr() as usize;
            let t = timestamp.as_ptr() as usize - string.as_ptr() as usize;

            return Ok((
                Range {
                    start: a,
                    end: a + address.len(),
                },
                Range {
                    start: t,
                    end: t + timestamp.len(),
                },
            ));
        }

        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "invalid beginning",
        ))
    }

    /// Create a new `Begin` from the given `String`.
    #[inline]
    pub fn new<T: Into<Vec<u8>>>(string: T) -> io::Result<Self> {
        let string = string.into();
        let (address, timestamp) = Begin::ranges(&string)?;

        Ok(Begin {
            // The parser verifies the content is US-ASCII, so it's safe.
            inner: unsafe { String::from_utf8_unchecked(string) },

            address,
            timestamp,
        })
    }

    /// The origin address, by RFC this can be any address ever used in any
    /// system at any time.
    #[inline]
    pub fn address(&self) -> &str {
        &self.inner[Range {
            start: self.address.start,
            end: self.address.end,
        }]
    }

    /// The timestamp.
    #[inline]
    pub fn timestamp(&self) -> &str {
        &self.inner[Range {
            start: self.timestamp.start,
            end: self.timestamp.end,
        }]
    }
}

mod parser {
    use crate::util::parser::{is_printable, is_printable_or_ws, is_ws};
    use nom::bytes::complete::{tag, take_while, take_while1};
    use nom::sequence::tuple;
    use nom::IResult;

    pub fn parse(input: &[u8]) -> IResult<&[u8], (&[u8], &[u8])> {
        let (input, (_, _, address, _, timestamp)) = tuple((
            tag("From"),
            take_while1(is_ws),
            address,
            take_while1(is_ws),
            timestamp,
        ))(input)?;
        Ok((input, (address, timestamp)))
    }

    pub fn address(input: &[u8]) -> IResult<&[u8], &[u8]> {
        take_while(is_printable)(input)
    }

    pub fn timestamp(input: &[u8]) -> IResult<&[u8], &[u8]> {
        take_while(is_printable_or_ws)(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ok() {
        let v = Begin::new("From foo@example.com Wed Nov 17 14:35:53 2010").unwrap();
        assert_eq!(v.address(), "foo@example.com");
        assert_eq!(v.timestamp(), "Wed Nov 17 14:35:53 2010");
    }

    #[test]
    fn ok_gmail() {
        let v = Begin::new("From 1668703170433825012@xxx Fri Jun 05 23:22:35 +0000 2020").unwrap();
        assert_eq!(v.address(), "1668703170433825012@xxx");
        assert_eq!(v.timestamp(), "Fri Jun 05 23:22:35 +0000 2020");
    }
}
