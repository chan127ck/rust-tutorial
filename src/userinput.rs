// ignore unused variable error
#![allow(unused)]

// standard input output
use std::io;
// random range
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
// ordering
use std::cmp::Ordering;
    
pub fn io_func() {
    println!("What is your name?");
    // mutatable variable, value can change
    let mut name = String::new();
    let greeting: &str = "Nice to meet you";
    // take input from user with reference mutable variable "name"
    io::stdin().read_line(&mut name)
        .expect(r#"Didn't receive Input"#);
    println!("Hello {}, {}", name.trim_end(), greeting);
}
