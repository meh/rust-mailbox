extern crate mailbox;
use mailbox::{mail, header};

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

	for mail in mail::read(File::open(env::args().nth(1).expect("no file given")).unwrap()).body(false) {
		if let Ok(mail) = mail {
			status.total += 1;

			for field in vec![mail.headers().get_from::<header::Status, _>("Status"), mail.headers().get_from::<header::Status, _>("X-Status")] {
				if let Some(Ok(s)) = field {
					if s.contains(header::status::SEEN) {
						status.seen += 1;
					}

					if s.contains(header::status::OLD) {
						status.old += 1;
					}

					if s.contains(header::status::ANSWERED) {
						status.answered += 1;
					}

					if s.contains(header::status::FLAGGED) {
						status.flagged += 1;
					}

					if s.contains(header::status::DRAFT) {
						status.draft += 1;
					}

					if s.contains(header::status::DELETED) {
						status.deleted += 1;
					}
				}
			}
		}
	}

	println!("{:#?}", status);
}
