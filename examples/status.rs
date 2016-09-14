extern crate mailbox;
use mailbox::header;

use std::fs::File;
use std::env;

#[derive(Eq, PartialEq, Copy, Clone, Default, Debug)]
pub struct Status {
	pub total:    usize,
	pub seen:     usize,
	pub old:      usize,
	pub answered: usize,
	pub flagged:  usize,
	pub draft:    usize,
	pub deleted:  usize,
}

fn main() {
	let mut status = Status::default();

	for path in env::args().skip(1) {
		for mail in mailbox::read(File::open(path).unwrap()).body(false) {
			if let Ok(mail) = mail {
				let mut current = header::Status::empty();

				if let Some(Ok(s)) = mail.headers().get::<header::Status>() {
					current |= s;
				}

				if let Some(Ok(s)) = mail.headers().get_from::<header::Status, _>("X-Status") {
					current |= s;
				}

				status.total += 1;

				if current.contains(header::status::SEEN) {
					status.seen += 1;
				}

				if current.contains(header::status::OLD) {
					status.old += 1;
				}

				if current.contains(header::status::ANSWERED) {
					status.answered += 1;
				}

				if current.contains(header::status::FLAGGED) {
					status.flagged += 1;
				}

				if current.contains(header::status::DRAFT) {
					status.draft += 1;
				}

				if current.contains(header::status::DELETED) {
					status.deleted += 1;
				}
			}
		}
	}

	println!("{:#?}", status);
}
