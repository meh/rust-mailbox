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

use std::collections::{hash_map, HashMap};
use case::CaseExt;
use super::Header;

#[derive(Clone, Debug)]
pub struct Headers(HashMap<String, Vec<String>>);

impl Headers {
	#[inline]
	pub fn new() -> Self {
		Headers(HashMap::new())
	}

	#[inline]
	#[doc(hidden)]
	pub fn insert<T: AsRef<str>>(&mut self, key: T, value: String) {
		self.0.entry(key.as_ref().to_camel()).or_insert(Vec::new()).push(value);
	}

	#[inline]
	pub fn get<T: AsRef<str>>(&self, key: T) -> Option<Result<Header, &[String]>> {
		let key = key.as_ref().to_camel();

		if let Some(slice) = self.0.get(&key) {
			Some(Header::parse(&key, slice).map_err(|_| slice.as_ref()))
		}
		else {
			None
		}
	}

	#[inline]
	pub fn len(&self) -> usize {
		self.0.len()
	}

	#[inline]
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	#[inline]
	pub fn contains_key<T: AsRef<str>>(&self, key: T) -> bool {
		self.0.contains_key(&key.as_ref().to_camel())
	}

	#[inline]
	pub fn keys(&self) -> hash_map::Keys<String, Vec<String>> {
		self.0.keys()
	}

	#[inline]
	pub fn iter(&self) -> Iter {
		Iter(self.0.iter())
	}
}

impl<'a> IntoIterator for &'a Headers {
	type Item     = (&'a str, Result<Header, &'a [String]>);
	type IntoIter = Iter<'a>;

	#[inline]
	fn into_iter(self) -> Iter<'a> {
		self.iter()
	}
}

pub struct Iter<'a>(hash_map::Iter<'a, String, Vec<String>>);

impl<'a> Iterator for Iter<'a> {
	type Item = (&'a str, Result<Header, &'a [String]>);

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		if let Some((key, slice)) = self.0.next() {
			Some((key, Header::parse(key, slice).map_err(|_| slice.as_ref())))
		}
		else {
			None
		}
	}
}
