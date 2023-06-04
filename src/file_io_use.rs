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

// Result has 2 variants Ok and Err
// enum Result<T,E> {
//    Ok(T),
//    Err(E),
// }
// Where T represents the data type of the value returns 
// and E the type of error
pub fn file_io_example() {
    // create file.txt
    let path = "lines.txt";
    let output = File::create(path);
    let mut output = match output {
        Ok(file) => file,
        Err(error) => {panic!("problem creating file : {:?}", error)},
    };
    // write to file.txt
    write!(output, "Just some random words").expect(
        "fail to write to file"
    );
    // read line from file.txt
    // unwrap is to get the content from the Result
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }
    // handle error in opening a file
    let output2 = File::open("rand.txt");
    let output2 = match output2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("rand.txt"){
                Ok(fc) => fc,
                Err(e) => panic!("Can't create file for some unkown reason")
            },
            _other_error => panic!("Problem opening the file"),
        }
    };
}
