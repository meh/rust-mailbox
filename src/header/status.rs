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

bitflags! {
    pub struct Status: u8 {
        const NEW      = 0b00000001;
        const SEEN     = 0b00000010;
        const OLD      = 0b00000100;
        const ANSWERED = 0b00001000;
        const FLAGGED  = 0b00010000;
        const DRAFT    = 0b00100000;
        const DELETED  = 0b01000000;
    }
}

impl Header for Status {
    #[inline]
    fn name() -> &'static str {
        "Status"
    }

    #[inline]
    fn parse(values: &[header::Item]) -> io::Result<Self> {
        let mut status = Status::empty();

        for ch in values[0].chars() {
            status |= match ch {
                'N' => Status::NEW,
                'R' => Status::SEEN,
                'O' => Status::OLD,
                'A' => Status::ANSWERED,
                'F' => Status::FLAGGED,
                'T' => Status::DRAFT,
                'D' => Status::DELETED,

                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "invalid status",
                    ))
                }
            }
        }

        Ok(status)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::header::Header;
    use crate::stream::entry::header;

    macro_rules! parse {
        ($str:expr) => {
            <Status as Header>::parse(&[header::item($str)])
        };
    }

    #[test]
    fn new() {
        assert_eq!(parse!("N").unwrap(), Status::NEW);
    }

    #[test]
    fn read() {
        assert_eq!(parse!("R").unwrap(), Status::SEEN);
    }

    #[test]
    fn old() {
        assert_eq!(parse!("O").unwrap(), Status::OLD);
    }

    #[test]
    fn answered() {
        assert_eq!(parse!("A").unwrap(), Status::ANSWERED);
    }

    #[test]
    fn flagged() {
        assert_eq!(parse!("F").unwrap(), Status::FLAGGED);
    }

    #[test]
    fn draft() {
        assert_eq!(parse!("T").unwrap(), Status::DRAFT);
    }

    #[test]
    fn deleted() {
        assert_eq!(parse!("D").unwrap(), Status::DELETED);
    }

    #[test]
    fn mixed() {
        assert_eq!(
            parse!("ROD").unwrap(),
            Status::SEEN | Status::OLD | Status::DELETED
        );
        assert_eq!(
            parse!("FTA").unwrap(),
            Status::FLAGGED | Status::DRAFT | Status::ANSWERED
        );
    }

    #[test]
    fn fail() {
        assert!(parse!("ANTANI").is_err());
    }
}
