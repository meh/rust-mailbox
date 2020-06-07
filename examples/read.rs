extern crate mailbox;

use std::env;
use std::fs::File;

fn main() {
    let mbox = mailbox::read(File::open(env::args().nth(1).expect("no file given")).unwrap());

    for mail in mbox {
        println!("{:?}", mail);
    }
}
