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

/// A line based entry in a mailbox.
///
/// Note that there are no allocations or copies from `stream::Lines`.
#[derive(Clone, Debug)]
pub enum Entry {
    /// The beginning of an email, includes the absolute offset from the input
    /// and the origin.
    Begin(u64, Begin),

    /// A header.
    Header(Header),

    /// A line of body.
    Body(Vec<u8>),

    /// The end of the email.
    End,
}

mod begin;
pub use self::begin::Begin;

pub(crate) mod header;
pub use self::header::Header;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn begin_ok() {
        let v = Begin::new("From foo@example.com Wed Nov 17 14:35:53 2010").unwrap();
        assert_eq!(v.address(), "foo@example.com");
        assert_eq!(v.timestamp(), "Wed Nov 17 14:35:53 2010");
    }

    #[test]
    fn begin_fail() {
        assert!(Begin::new("From foo@example.com").is_err());
    }

    #[test]
    fn header_ok() {
        let v = Header::new("From: meh. <meh@schizofreni.co>").unwrap();
        assert_eq!(&*v.key(), "From");
        assert_eq!(&*v.value(), "meh. <meh@schizofreni.co>");
    }

    #[test]
    fn header_fail() {
        assert!(Header::new("From foo@example.com").is_err());
    }
}
