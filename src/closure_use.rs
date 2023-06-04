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

pub fn closure_example() {
    // let var_name = | parameters | -> return_type {BODY}
    let can_vote = |age: i32| {
        age >= 18
    };
    println!("Can vote: {}", can_vote(8));
    let mut samp1 = 5;
    // closure without param
    let print_val = || println!("samp1: {}", samp1);
    print_val();
    samp1 = 10;
    // mutable clousre, the value changes with param
    let mut change_var = || samp1 += 1;
    change_var();
    println!("samp1 = {}", samp1);
    samp1 = 10;
    println!("samp1 = {}", samp1);
    // use function in closure
    fn use_func<T>(a: i32, b:i32, func: T) -> i32 
    where T: Fn(i32, i32) -> i32 {
        func(a, b)
    }
    let sum = |x, y| x+y;
    let prod = |x, y| x*y;
    println!("5 + 4 = {}", use_func(5, 4, sum));
    println!("5 * 4 = {}", use_func(5, 4, prod));
}