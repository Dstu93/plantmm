
#[macro_use]
extern crate nom;

mod model;
mod parser;

#[cfg(test)]
mod tests;

use std::fs::File;
use std::io::Read;

fn main() {
    println!("Hello, world!");

    let mut textfile = File::open("src/tests/test_mindmap.txt").expect("file could not be found");
    let mut content = String::new();
    textfile.read_to_string(&mut content);

    let result = mm_body(&*content);
    println!("{:#?}",result);
}

// parser functions

named!(mm_body(&str) -> &str,
    error_position!(delimited!(
        tag!("@start"),
        take_until!("@end"),
        tag!("@end")
        )));
