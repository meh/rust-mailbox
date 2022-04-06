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

use casing::Casing;
use owning_ref::OwningRef;
use std::borrow::Cow;
use std::io;
use std::ops::Range;
use std::rc::Rc;

/// A header in an email.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Header {
    inner: Item,

    key: Range<usize>,
    value: Range<usize>,
}

/// A header item.
///
/// Note this is the same `String` that was allocated by the `stream::Lines` iterator,
/// this means there are no allocations or copies when accessing `key()` and `value()`.
pub type Item = OwningRef<Rc<String>, str>;

#[inline(always)]
pub(crate) fn item<T: Into<String>>(string: T) -> Item {
    OwningRef::new(Rc::new(string.into())).map(|s| s.as_ref())
}

impl Header {
    #[inline]
    pub(crate) fn ranges<T: AsRef<[u8]>>(string: T) -> io::Result<(Range<usize>, Range<usize>)> {
        let string = string.as_ref();

        if let Ok((_, (key, value))) = parser::parse(string) {
            let k = key.as_ptr() as usize - string.as_ptr() as usize;
            let v = value.as_ptr() as usize - string.as_ptr() as usize;

            Ok((
                Range {
                    start: k,
                    end: k + key.len(),
                },
                Range {
                    start: v,
                    end: v + value.len(),
                },
            ))
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid header",
            ))
        }
    }

    /// Create a new `Header` from the given `String`.
    #[inline]
    pub fn new<T: Into<Vec<u8>>>(string: T) -> io::Result<Self> {
        let string = string.into();
        let (key, value) = Header::ranges(&string)?;

        Ok(Header {
            // The parser verifies the content is US-ASCII, so it's safe.
            inner: item(unsafe { String::from_utf8_unchecked(string) }),

            key,
            value,
        })
    }

    /// The header key in the proper case.
    ///
    /// Note that this allocates only if the key is not already in the proper case.
    #[inline]
    pub fn key(&self) -> Item {
        match (&self.inner[Range {
            start: self.key.start,
            end: self.key.end,
        }])
            .header(Default::default())
        {
            Cow::Borrowed(_) => self.inner.clone().map(|s| {
                &s[Range {
                    start: self.key.start,
                    end: self.key.end,
                }]
            }),
            Cow::Owned(string) => OwningRef::new(Rc::new(string)).map(|s| s.as_ref()),
        }
    }

    /// The header value.
    #[inline]
    pub fn value(&self) -> Item {
        self.inner.clone().map(|s| {
            &s[Range {
                start: self.value.start,
                end: self.value.end,
            }]
        })
    }
}

mod parser {
    use crate::util::parser::{is_printable_no_colon, is_printable_or_ws, is_ws};
    use nom::bytes::complete::take_while;
    use nom::character::complete::char;
    use nom::sequence::tuple;
    use nom::IResult;

    pub fn parse(input: &[u8]) -> IResult<&[u8], (&[u8], &[u8])> {
        let (input, (key, _, _, value)) = tuple((key, char(':'), take_while(is_ws), value))(input)?;
        Ok((input, (key, value)))
    }

    pub fn key(input: &[u8]) -> IResult<&[u8], &[u8]> {
        take_while(is_printable_no_colon)(input)
    }

    pub fn value(input: &[u8]) -> IResult<&[u8], &[u8]> {
        take_while(is_printable_or_ws)(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ok() {
        let v = Header::new("From: meh. <meh@schizofreni.co>").unwrap();
        assert_eq!(&*v.key(), "From");
        assert_eq!(&*v.value(), "meh. <meh@schizofreni.co>");
    }

    #[test]
    fn fail() {
        assert!(Header::new("From foo@example.com").is_err());
    }
}
