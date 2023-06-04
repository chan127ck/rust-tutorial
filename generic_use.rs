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

use std::ops::Add;
// add unkown data type, need a specific function Add
fn get_sum_gen<T: Add<Output = T>>(x:T,y:T) -> T {
    return x + y;
}


pub fn generic_example() {
    println!("5 + 4 = {}", get_sum_gen(5, 4));
    println!("5.2 + 4.3 = {}", get_sum_gen(5.2, 4.3));
    
}
