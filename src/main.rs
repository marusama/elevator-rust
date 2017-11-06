extern crate regex;

use std::io;
use std::io::BufRead;
use regex::Regex;


fn main() {
    println!("Hello, world!");

    let num_re = Regex::new(r"^\d+$").unwrap();
    let num_num_re = Regex::new(r"^\d+-\d+$").unwrap();

    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let s = line.unwrap();
        if let "q" = s.as_ref() {
            break;
        }
        if num_re.is_match(&s) {
            println!("number");
        } else if num_num_re.is_match(&s) {
            println!("number-number");
        } else {
            println!("none");
        }
    }
}
