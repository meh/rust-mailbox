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

pub mod entry;
pub use self::entry::Entry;

mod lines;
pub use self::lines::Lines;

mod iter;
pub use self::iter::Iter;

use std::io::{BufReader, Read};

/// Create an `Iterator` over line based entries.
#[inline]
pub fn entries<R: Read>(input: R) -> Iter<R> {
    Iter::new(input)
}

/// Create an `Iterator` over lines.
#[inline]
pub fn lines<R: Read>(input: R) -> Lines<BufReader<R>> {
    Lines::new(BufReader::new(input))
}
