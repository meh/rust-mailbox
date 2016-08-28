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

extern crate case;
extern crate regex;
extern crate chrono;
extern crate mime;

extern crate nix;

pub mod stream;

pub mod mail;
pub use mail::Mail;

use std::io::Read;

pub fn read<R: Read>(input: R) -> mail::Iter<R> {
	mail::read(input)
}
