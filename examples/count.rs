extern crate mailbox;

use std::fs::File;
use std::env;

fn main() {
	let mbox = mailbox::read(File::open(env::args().nth(1).expect("no file given")).unwrap());

	let mut read  = 0;
	let mut total = 0;

	for mail in mbox {
		if let Ok(mail) = mail {
			total += 1;

			if let Some(&mailbox::mail::Header::Status(status)) = mail.headers().get("Status") {
				if status.contains(mailbox::mail::status::SEEN) {
					read += 1;
				}
			}
		}
	}

	println!("{}/{}", read, total);
}
