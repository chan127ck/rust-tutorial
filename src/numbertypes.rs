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
    
pub fn num_type() {
    // Unsigned integer: u8, u16, u32, u64, u128, usize
    // Signed integer: 8, i16, i32, i64, i128, isize
    println!("Max u32: {}", u32::MAX);
    println!("Max u64: {}", u64::MAX);
    println!("Max u128: {}", u128::MAX);
    println!("Max usize: {}", usize::MAX);
    println!("Max f32: {}", f32::MAX);
    // boolean
    let is_true: bool = true;
    // won't throw error with beginning "_"
    let _is_false: bool = false;
    // char is single quote
    // sting is doble quote
    let my_grade: char = 'A';
    let my_name: &str = "Kai";
    // math
    let num_1: f32 = 1.11111111111111;
    println!("f32: {}", num_1 + 0.11111111111111);
    let num_2: f64 = 1.11111111111111;
    println!("f64: {}", num_2 + 0.11111111111111);
    let num_3: u32 = 5;
    let num_4: u32 = 4;
    println!("5 + 4 = {}", num_3 + num_4);
    println!("5 - 4 = {}", num_3 - num_4);
    println!("5 * 4 = {}", num_3 * num_4);
    println!("5 / 4 = {}", num_3 / num_4);
    println!("5 % 4 = {}", num_3 % num_4);
    // mutable number
    let mut num_0 = 1;
    num_0 += 1;
    println!("num_0 = {}", num_0);
    // random number
    // generate random number from 1 to 100
    let random_num: i32 = rand::thread_rng().gen_range(1..101);
    println!("Random number = {}", random_num);
}
