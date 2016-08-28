extern crate mailbox;

use std::fs::File;
use std::env;

fn main() {
	let mbox = mailbox::read(File::open(env::args().nth(1).expect("no file given")).unwrap());

	for mail in mbox {
		println!("{:?}", mail);
	}
}
