extern crate mailbox;

use std::fs::File;
use std::env;

fn main() {
	let stream = mailbox::stream::read(File::open(env::args().nth(1).expect("no file given")).unwrap());

	for entry in stream {
		println!("{:?}", entry);
	}
}
