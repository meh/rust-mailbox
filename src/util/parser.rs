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

const NONE: u8 = 0b000;
const PRINT: u8 = 0b001;
const COLON: u8 = 0b010;
const SPACE: u8 = 0b100;

// Ugly table of DOOM, gotta run and gun.
static ASCII: [u8; 256] = [
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    SPACE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    SPACE,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT | COLON,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    PRINT,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
    NONE,
];

#[inline(always)]
pub fn is_ws(ch: u8) -> bool {
    unsafe { ASCII.get_unchecked(ch as usize) & SPACE != 0 }
}

#[inline(always)]
pub fn is_printable(ch: u8) -> bool {
    unsafe { ASCII.get_unchecked(ch as usize) & PRINT != 0 }
}

#[inline(always)]
pub fn is_printable_or_ws(ch: u8) -> bool {
    unsafe { ASCII.get_unchecked(ch as usize) & (PRINT | SPACE) != 0 }
}

#[inline(always)]
pub fn is_printable_no_colon(ch: u8) -> bool {
    unsafe { ASCII.get_unchecked(ch as usize) & (PRINT | COLON) == PRINT }
}

macro_rules! take_while_n {
	($input:expr, $n:expr, $submac:ident!( $($args:tt)* )) => ({
		let count = $n;

		if $input.len() < count {
			return Err($crate::nom::Err::Incomplete($crate::nom::Needed::Size(count)));
		}

		match $input.iter().take(count).position(|c| !$submac!(*c, $($args)*)) {
			Some(n) => {
				let res:$crate::nom::IResult<_,_> = if n == count {
					Ok((&$input[n..], &$input[..n]))
				}
				else {
					Err($crate::nom::Err::Error(error_position!(&$input[n..], $crate::nom::error::ErrorKind::Tag)))
				};

				res
			},

			None => {
				Ok((&$input[($input).len()..], $input))
			}
		}
	});

	($input:expr, $n:expr, $f:expr) => (
		take_while_n!($input, $n, call!($f));
	);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ws() {
        assert!(is_ws(b' '));
        assert!(!is_ws(b'a'));
    }

    #[test]
    fn printable() {
        assert!(is_printable(b'a'));
        assert!(!is_printable(b' '));
    }

    #[test]
    fn printable_or_ws() {
        assert!(is_printable_or_ws(b'a'));
        assert!(is_printable_or_ws(b' '));
        assert!(is_printable_or_ws(b'\t'));
    }

    #[test]
    fn printable_no_colon() {
        assert!(is_printable_no_colon(b'a'));
        assert!(!is_printable_no_colon(b':'));
        assert!(!is_printable_no_colon(b' '));
    }
}
