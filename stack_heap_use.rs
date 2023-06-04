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

// Stack: Stores values in a last in first out format
// Data on the stack must have a defined fixed size

// Heap: When putting data on the heap you request a
// certain amount of space. the OS finds space available
// and return an address for the space called a pointer.

// RULES:
//      1. Each value has a variable that's called its owner
//      2. There is only one owner at a time
//      3. When the owner goes out of scope the value disappers

fn print_str(x: String){
    println!("A string {}", x);
}

fn print_return_str(x: String) -> String{
    println!("A string {}", x);
    x
}

fn change_string(name: &mut String) {
    name.push_str(" is happy");
    println!("Message: {}", name);
}

pub fn stack_heap_example() {
    // str2 is assigned st1, str 1 is gone
    // let str1 = String::from("world");
    // let str2 = str1;
    // println!("Hello {}", str1);
    // correct method, use clone
    let str1 = String::from("world");
    let str2 = str1.clone();
    println!("Hello {}", str1);
    // str3 use str1, to ensure only one owner, str1 is gone.
    // so first statement need .clone
    print_str(str1.clone());
    let str3 = print_return_str(str1);
    println!("{}", str3);
    // str4 need to be mut as 'push_str' alters the str4
    let mut str4 = String::from("Derek");
    change_string(&mut str4);
}
