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
    
pub fn variable_type() {
    // constant, "_" can represent "," in dollar format
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.1415;
    let age: &str = "21";
    // shawdowing: variable with same name but different data types
    let mut age: u32 = age.trim().parse()
        .expect("Age wasn't assigned a number");
    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_MIL);


}
