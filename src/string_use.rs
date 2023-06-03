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
    
pub fn string_example() {
    let mut st1 =String::new();
    //add a char to the string
    st1.push('A');
    //add a word to the string
    st1.push_str(" word");
    //print each word of the string
    for word in st1.split_whitespace() {
        println!("{}", word);
    }
    //replace word
    let st2 = st1.replace("A", "Another");
    println!("{}", st2);
    //find distinct char in a string
    println!("--------find distinct char in a string--------");
    let st3 = String::from("x r t b h k k a m c");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1{
        println!("{}", char)
    }
    //&str to String conversion
    let st4: &str = "Random String";
    let mut st5: String= st4.to_string();
    println!("{}", st5);
    //change string to bytes
    let byte_arr1 =st5.as_bytes();
    println!("bytes: {:?}", byte_arr1);
    //slice a string
    let st6 = &st5[0..6];
    println!("Sliced string: {}", st6);
    //length of string
    println!("String length: {}", st6.len());
    //empty the string
    st5.clear();
    let st7 = String::from("Just some");
    let st8 = String::from(" words");
    // use reference in st8 but not st7, result: st7 is deleted and st8 still exist
    let st9 = st7 + &st8;
    println!("Comined string: {}", st9);
    for char in st9.bytes() {
        println!("Unicode Char: {}", char);
    }
}
