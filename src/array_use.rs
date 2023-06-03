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
    
pub fn array_example() {
    // array only can have same data type in the list
    let arr_1 = [1,2,3,4,5];
    println!("1st: {}", arr_1[0]);
    println!("Length: {}", arr_1.len());
    // loop through the element
    println!("-------loop through the element-------");
    let mut loop_idx = 0;
    loop {
        if {arr_1[loop_idx] % 2 == 0} {
            // skip even number
            loop_idx += 1;
            continue;
        }
        if {arr_1[loop_idx] == 5} {
            // break the loop
            break;
        }
        println!("Val: {}", arr_1[loop_idx]);
        loop_idx += 1;
    }
    // while loop
    println!("-------While loop-------");
    loop_idx = 0;
    while loop_idx < arr_1.len() {
        println!("Arr: {}", arr_1[loop_idx]);
        loop_idx += 1;
    }
    // for loop
    println!("-------for loop-------");
    for val in arr_1.iter() {
        println!("Val {}", val);
    }
}
