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

use std::slice;

#[derive(PartialEq, Eq, Clone, Default, Debug)]
pub struct Body(Vec<Vec<u8>>);

impl Body {
	#[doc(hidden)]
	#[inline]
	pub fn append(&mut self, data: Vec<u8>) {
		self.0.push(data);
	}

	#[inline]
	pub fn len(&self) -> usize {
		self.0.iter().map(|v| v.len()).sum::<usize>() + self.0.len() * 2
	}

	#[inline]
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	#[inline]
	pub fn iter(&self) -> Iter {
		Iter {
			parent:    self.0.iter(),
			child:     None,
			separator: Separator::None,
		}
	}
}

impl<'a> IntoIterator for &'a Body {
	type Item     = u8;
	type IntoIter = Iter<'a>;

	#[inline]
	fn into_iter(self) -> Iter<'a> {
		self.iter()
	}
}

pub struct Iter<'a> {
	parent:    slice::Iter<'a, Vec<u8>>,
	child:     Option<slice::Iter<'a, u8>>,
	separator: Separator,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Separator {
	CarriageReturn,
	LineFeed,
	None,
}

impl<'a> Iterator for Iter<'a> {
	type Item = u8;

	fn next(&mut self) -> Option<Self::Item> {
		loop {
			match self.separator {
				Separator::CarriageReturn => {
					self.separator = Separator::LineFeed;
					return Some(b'\r');
				}

				Separator::LineFeed => {
					self.separator = Separator::None;
					return Some(b'\n');
				}

				Separator::None => {
					if self.child.is_some() {
						if let Some(&byte) = self.child.as_mut().unwrap().next() {
							return Some(byte);
						}
						else {
							self.child     = None;
							self.separator = Separator::CarriageReturn;
						}
					}
					else {
						self.child = self.parent.next().map(|v| v.iter());

						if self.child.is_none() {
							return None;
						}
					}
				}
			}
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn len() {
		let mut body = Body::default();
		body.append(vec![1, 2, 3]);
		body.append(vec![4]);
		body.append(vec![5, 6]);

		assert_eq!(body.len(), 12);
	}

	#[test]
	fn iter() {
		let mut body = Body::default();
		body.append(vec![1, 2, 3]);
		body.append(vec![4]);
		body.append(vec![5, 6]);

		assert_eq!(body.iter().collect::<Vec<u8>>(), vec![1, 2, 3, b'\r', b'\n', 4, b'\r', b'\n', 5, 6, b'\r', b'\n']);
	}
}
