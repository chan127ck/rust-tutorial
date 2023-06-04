// ignore unused variable error
#![allow(unused)]

use core::num;
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

// rust doesn't have exception
pub fn error_handling_example() {
    panic!("Terrible error");
    // index error
    let lil_arr = [1,2];
    println!("{}", lil_arr[10]);
}
