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

use std::io::{self, BufRead};

/// Iterator over ASCII lines.
///
/// The content of a line is not assumed to be in any specific encoding.
pub struct Lines<R: BufRead>(R, u64);

impl<R: BufRead> Lines<R> {
    /// Create a new `Iterator` from the given input.
    #[inline]
    pub fn new(input: R) -> Self {
        Lines(input, 0)
    }
}

impl<R: BufRead> Iterator for Lines<R> {
    type Item = io::Result<(u64, Vec<u8>)>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = Vec::new();
        let offset = self.1;

        match self.0.read_until(b'\n', &mut buffer) {
            Ok(0) => None,

            Ok(_) => {
                self.1 += buffer.len() as u64;

                if buffer.last() == Some(&b'\n') {
                    buffer.pop();

                    if buffer.last() == Some(&b'\r') {
                        buffer.pop();
                    }
                }

                Some(Ok((offset, buffer)))
            }

            Err(e) => Some(Err(e)),
        }
    }
}
