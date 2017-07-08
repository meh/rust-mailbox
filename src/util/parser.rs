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

pub const WSP: &'static [u8] = b" \t";

const NONE:  u8 = 0b000;
const PRINT: u8 = 0b001;
const COLON: u8 = 0b010;
const SPACE: u8 = 0b100;

// Ugly table of DOOM, gotta run and gun.
static ASCII: [u8; 256] = [
	NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE,
	NONE, SPACE, NONE, NONE, NONE, NONE, NONE, NONE,
	NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE,
	NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE,
	SPACE, PRINT, PRINT, PRINT, PRINT, PRINT, PRINT, PRINT,
	PRINT, PRINT, PRINT, PRINT, PRINT, PRINT, PRINT, PRINT,
	PRINT, PRINT, PRINT, PRINT, PRINT, PRINT, PRINT, PRINT,
	PRINT, PRINT, PRINT | COLON, PRINT, PRINT, PRINT, PRINT, PRINT,
	PRINT, PRINT, PRINT, PRINT, PRINT, PRINT, PRINT, PRINT,
	PRINT, PRINT, PRINT, PRINT, PRINT, PRINT, PRINT, PRINT,
	PRINT, PRINT, PRINT, PRINT, PRINT, PRINT, PRINT, PRINT,
	PRINT, PRINT, PRINT, PRINT, PRINT, PRINT, PRINT, PRINT,
	PRINT, PRINT, PRINT, PRINT, PRINT, PRINT, PRINT, PRINT,
	PRINT, PRINT, PRINT, PRINT, PRINT, PRINT, PRINT, PRINT,
	PRINT, PRINT, PRINT, PRINT, PRINT, PRINT, PRINT, PRINT,
	PRINT, PRINT, PRINT, PRINT, PRINT, PRINT, PRINT, NONE,
	NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE,
	NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE,
	NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE,
	NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE,
	NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE,
	NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE,
	NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE,
	NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE,
	NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE,
	NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE,
	NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE,
	NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE,
	NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE,
	NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE,
	NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE,
	NONE, NONE, NONE, NONE, NONE, NONE, NONE, NONE,
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

macro_rules! take_until_either_or_eof {
	($i:expr, $inp:expr) => ({
		#[inline(always)]
		fn as_bytes<T: $crate::nom::AsBytes>(b: &T) -> &[u8] {
			b.as_bytes()
		}

		let expected   = $inp;
		let bytes      = as_bytes(&expected);
		take_until_either_bytes_or_eof!($i, bytes)
	});
}

macro_rules! take_until_either_bytes_or_eof {
	($i:expr, $bytes:expr) => ({
		let res: $crate::nom::IResult<_,_> = if 1 > $i.len() {
			$crate::nom::IResult::Incomplete($crate::nom::Needed::Size(1))
		}
		else {
			let mut index  = 0;
			let mut parsed = false;

			for idx in 0..$i.len() {
				if idx + 1 > $i.len() {
					index = idx;
					break;
				}
				for &t in $bytes.iter() {
					if $i[idx] == t {
						parsed = true;
						index = idx;
						break;
					}
				}
				if parsed { break; }
			}

			if parsed {
				$crate::nom::IResult::Done(&$i[index..], &$i[0..index])
			}
			else {
				$crate::nom::IResult::Done(b"", &$i[..])
			}
		};

		res
	});
}

macro_rules! take_while_n {
	($input:expr, $n:expr, $submac:ident!( $($args:tt)* )) => ({
		let count = $n;

		if $input.len() < count {
			return $crate::nom::IResult::Incomplete($crate::nom::Needed::Size(count));
		}

		match $input.iter().take(count).position(|c| !$submac!(*c, $($args)*)) {
			Some(n) => {
				let res:$crate::nom::IResult<_,_> = if n == count {
					$crate::nom::IResult::Done(&$input[n..], &$input[..n])
				}
				else {
					$crate::nom::IResult::Error($crate::nom::ErrorKind::Tag)
				};

				res
			},

			None => {
				$crate::nom::IResult::Done(&$input[($input).len()..], $input)
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
