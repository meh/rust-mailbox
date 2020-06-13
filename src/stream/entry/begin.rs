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
            if timestamp.len() == 24 {
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

    named!(pub parse(&[u8]) -> (&[u8], &[u8]),
		do_parse!(
			tag!("From ") >>
			take_while!(is_ws) >>
			addr: address >>
			take_while!(is_ws) >>
			time: timestamp >>

			(addr, time)));

    named!(address(&[u8]) -> &[u8],
		take_while!(is_printable));

    named!(timestamp(&[u8]) -> &[u8],
		take_while_n!(24, is_printable_or_ws));
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
    fn fail() {
        assert!(Begin::new("From foo@example.com").is_err());
        assert!(Begin::new("From foo@example.com Wed Nov 17 14:35:53 20109").is_err());
    }
}
