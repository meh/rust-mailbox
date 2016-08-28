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
use std::str::FromStr;
use std::ops::Deref;
use chrono::{DateTime, FixedOffset};

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Date(DateTime<FixedOffset>);

impl FromStr for Date {
	type Err = io::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		DateTime::parse_from_rfc2822(s)
			.map(|v| Date(v))
			.map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid date"))
	}
}

impl Deref for Date {
	type Target = DateTime<FixedOffset>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
