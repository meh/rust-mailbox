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

use std::collections::HashMap;
use case::CaseExt;
use super::Header;

#[derive(Clone, Debug)]
pub struct Headers(HashMap<String, Vec<String>>);

impl Headers {
	pub fn new() -> Self {
		Headers(HashMap::new())
	}

	pub fn insert<T: AsRef<str>>(&mut self, key: T, value: String) {
		self.0.entry(key.as_ref().to_camel()).or_insert(Vec::new()).push(value);
	}

	pub fn get<T: AsRef<str>>(&self, key: T) -> Option<Result<Header, &[String]>> {
		let key = key.as_ref().to_camel();

		if let Some(slice) = self.0.get(&key) {
			Some(Header::parse(&key, slice).map_err(|_| slice.as_ref()))
		}
		else {
			None
		}
	}
}
