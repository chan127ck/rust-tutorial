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
    
pub fn match_example() {
    // example
    let age2: i32 = 8;
    match age2{
        // from 1 to and include 18
        1..=18 => println!("Important birthday"),
        // 21 or 50
        21 | 50 => println!("Important birthday"),
        // 65 to and include maximum of i32
        65..=i32::MAX => println!("Important birthday"),
        // other values
        _ => println!("Not an Important birthday")
    };
    // Using cmp (compare) and ordering
    let my_age = 18;
    let voting_age = 18;
    match my_age.cmp(&voting_age){
        Ordering::Less => println!("Can't vote"),
        Ordering::Greater => println!("Can vote"),
        Ordering::Equal => println!("You gain the right to vote"),
    };
}
