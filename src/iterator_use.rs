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

pub fn iterator_example() {
    let mut arr_it = [1,2,3,4];
    for val in arr_it.iter() {
        println!("{}", val);
    }
    let v = [1, 2, 3];
    let mut iter = v.into_iter();
    // the result is Some(1)
    println!("First value of iterator: {:?}", iter.next());
    // another method to iterate iterator
    let mut iter1 = arr_it.iter();
    println!("1st : {:?}", iter1.next());
    
}