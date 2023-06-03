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
    
pub fn if_example() {
    // example
    let age: i32 = 8;
    if (age >= 1) && (age <= 18){
        println!("Important Birthday");
    } 
    else if {age == 21} || (age == 50) {
        println!("Important Birthday");
    }
    else if age >= 65 {
        println!("Important Birthday");
    }
    else {
        println!("Not an Important Birthday");
    }

    let mut my_age = 47;
    let can_vote = if my_age >= 18 {true} else {false};
    println!("Can vote: {}", can_vote);
}
