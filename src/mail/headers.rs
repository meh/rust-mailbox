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
use casing::Casing;
use super::Header;
use stream::entry;

#[derive(Clone, Default, Debug)]
pub struct Headers(HashMap<entry::header::Key, Vec<entry::Header>>);

impl Headers {
	#[inline]
	#[doc(hidden)]
	pub fn insert(&mut self, value: entry::Header) {
		self.0.entry(value.key()).or_insert_with(Vec::new).push(value);
	}

	#[inline]
	pub fn get<T: AsRef<str>>(&self, key: T) -> Option<Result<Header, Vec<&str>>> {
		let key = key.as_ref().header(Default::default());

		if let Some(slice) = self.0.get(key.as_ref()) {
			Some(Header::parse(key.as_ref(), slice).map_err(|_| slice.iter().map(|v| v.value()).collect()))
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
		self.0.contains_key(key.as_ref().header(Default::default()).as_ref())
	}

	#[inline]
	pub fn keys(&self) -> hash_map::Keys<entry::header::Key, Vec<entry::Header>> {
		self.0.keys()
	}

	#[inline]
	pub fn iter(&self) -> Iter {
		Iter(self.0.iter())
	}
}

impl<'a> IntoIterator for &'a Headers {
	type Item     = (&'a str, Result<Header, Vec<&'a str>>);
	type IntoIter = Iter<'a>;

	#[inline]
	fn into_iter(self) -> Iter<'a> {
		self.iter()
	}
}

pub struct Iter<'a>(hash_map::Iter<'a, entry::header::Key, Vec<entry::Header>>);

impl<'a> Iterator for Iter<'a> {
	type Item = (&'a str, Result<Header, Vec<&'a str>>);

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		if let Some((key, slice)) = self.0.next() {
			Some((key, Header::parse(key.as_ref(), slice).map_err(|_| slice.iter().map(|v| v.value()).collect())))
		}
		else {
			None
		}
	}
}
