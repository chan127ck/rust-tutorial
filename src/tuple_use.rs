// ignore unused variable error
#![allow(unused)]

// standard input output
use std::io;
use std::thread::Thread;
// random range
use rand::Rng;
use rand::rngs::ThreadRng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
// ordering
use std::cmp::Ordering;
    
pub fn tuple_example() {
    // tuple can have multiple data types
    let my_tuple: (u8, String, f64) = (47, "Derek".to_string(), 50_000.1);
    println!("Name: {}", my_tuple.1);
    let (v1, v2, v3) = my_tuple;
    println!("Age: {}", v1);
    
}
