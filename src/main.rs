
#[macro_use]
extern crate nom;

mod model;
mod parser;

#[cfg(test)]
mod tests;

use std::fs::File;
use std::io::Read;

fn main() {
    let mut textfile = File::open("src/tests/test_mindmap.txt").expect("file could not be found");
    let mut content = String::new();
    textfile.read_to_string(&mut content).unwrap();
    assert!(content.is_ascii());
    assert!(!content.is_empty());
    println!("Content String: {}\n\n", &*content);
    //let result = mm_body(&*content);
    let result = mm_body(&*content);
    println!("{:#?}",result);
}

// parser functions
named!(start<&str,&str> , tag_s!("@start"));
named!(mm_body(&str) -> &str,
take_until!("@start") ~
    delimited!(
        tag_s!("@start"),
        take_until!("@end"),
        tag_s!("@end")
        ));
