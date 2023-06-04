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

use std::collections::HashMap;

// hash map is to store key value pairs
pub fn hasp_map_example() {
    let mut heroes = HashMap::new();
    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Batman", "Bruce Wayne");
    heroes.insert("The Flash", "Barry Allen");

    // print key, value pair
    for (k, v) in heroes.iter() {
        println!("Key: {}, Val: {}", k, v);
    }
    //print length of hash map
    println!("Length: {}", heroes.len());
    // check a key in the hash map
    if heroes.contains_key(&"Batman") {
        let the_batman = heroes.get(&"Batman");
        match the_batman {
            Some(x) => println!("Batman is a hero"),
            None => println!("Batman is not a hero")
        }
    }
}
