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

pub fn vector_example() {
    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1,2,3,4];
    // add 5 to vector
    vec2.push(5);
    // print 1st element
    println!("1st: {}", vec2[0]);
    // if index error, print no second value
    let second: &i32 = &vec2[1];
    match vec2.get(1) {
        // Some represents the result of a computation that might have failed. 
        // If it did fail, it will return None (representing a null value), 
        // or Some (value) if it succeeded.
        Some(second) => println!("2nd : {}", second),
        None => println!("No second value"),
    }
    // multiply each element of vector by 2
    for i in &mut vec2 {
        // * is to dereference i
        *i *= 2
    }
    for i in &vec2 {
        println!("{}", i)
    }
    println!("Vec length: {}", vec2.len());
    // Vector return Some(value)
    println!("Pop: {:?}", vec2.pop());

}
