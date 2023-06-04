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

// generic function
fn say_hello(){
    println!("Hello");
}
// function with args
fn get_sum(x: i32, y:i32){
    println!("{} + {} ={}", x, y, x+y);
}
// function with return value
fn get_sum_2(x: i32, y:i32) -> i32 {
    return x + y;
}
// return 2 values
fn get_2(x: i32) -> (i32, i32) {
    return (x+1, x+2);
}
// pass in a number list
fn sum_list(list: &[i32]) -> i32 {
    let mut sum = 0;
    for &val in list.iter() {
        sum += &val;
    }
    return sum;
}

pub fn function_example() {
    say_hello();
    get_sum(4,5);
    println!("{}", get_sum_2(4,5));
    let (val1, val2) = get_2(4);
    println!("{}, {}", val1, val2);
    let num_list = vec![1,2,3,4,5];
    println!("Sum of list = {}", sum_list(&num_list));
}
