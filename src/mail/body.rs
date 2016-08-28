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

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Body {
	data: Vec<Vec<u8>>,
}

impl Body {
	#[inline]
	pub fn new() -> Self {
		Body {
			data: Vec::new(),
		}
	}

	#[inline]
	pub fn append(&mut self, data: Vec<u8>) {
		self.data.push(data);
	}
}
