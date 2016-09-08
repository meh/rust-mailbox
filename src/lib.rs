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

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate bitflags;
extern crate owning_ref;

#[macro_use]
extern crate nom;
extern crate casing;
extern crate chrono;
extern crate mime;

#[macro_use]
mod util;
pub use util::Address;

pub mod stream;

pub mod mail;
pub use mail::Mail;

use std::io::Read;

#[inline]
pub fn read<R: Read>(input: R) -> mail::Iter<R> {
	mail::read(input)
}
