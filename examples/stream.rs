extern crate mailbox;

use std::env;
use std::fs::File;

fn main() {
    let path = env::args().nth(1).expect("no file given");

    for entry in mailbox::stream::entries(File::open(path).unwrap()) {
        println!("{:?}", entry);
    }
}
