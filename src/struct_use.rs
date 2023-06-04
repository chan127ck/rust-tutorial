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



pub fn struct_example() {
    struct Customer {
        name: String,
        address: String,
        balance: f32,
    } 
    // create a customer
    let mut bob = Customer{
        name: String::from("bob"),
        address: String::from("1 Main St"),
        balance: 234.5
    };
    // change address
    bob.address = String::from("2 Main St");

    // example of struct with unknown data types
    struct Rectangle<T, U> {
        length: T,
        height: U
    }
    let rec = Rectangle{length: 4, height: 10.5};
    // trait example
    const PI: f32 = 3.14;
    trait Shape{
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }
    struct Circle {length: f32, width: f32};
    struct Rectangle2 {length: f32, width: f32};
    
    // implement Shape for Rectangle2
    impl Shape for Rectangle2{
        fn new(length: f32, width: f32) -> Rectangle2 {
            return Rectangle2{length, width};
        }
        fn area(&self) -> f32 {
            return self.length * self.width;
        }
    }
    impl Shape for Circle{
        fn new(length: f32, width: f32) -> Circle {
            return Circle{length, width};
        }
        fn area(&self) -> f32 {
            return (self.length/2.0).powf(2.0) * PI;
        }
    }
    let rec2: Rectangle2 = Shape::new(10.0, 10.0);
    let circ: Circle = Shape::new(10.0, 10.0);
    println!("Rec area: {}", rec2.area());
    println!("Circle area: {}", circ.area());
}
