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

// concurrency lib
use std::thread;
use std::time::Duration;
// smart pointer lib
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};

pub fn concurrency_example() {
    let thread1 = thread::spawn(|| {
        for i in 1..25 {
            println!("Spawned thread: {}", i);
            // sleep for 1 sec
            thread::sleep(Duration::from_millis(1));       
        }
    });
    for i in 1..20 {
        println!("Main thread: {}", i);
        // sleep for 1 sec
        thread::sleep(Duration::from_millis(1));
        
    }
    // ensure thread1 to complete
    thread1.join().unwrap();

    pub struct Bank {
        balance: f32
    }

    // Arc: provide share ownership of same value
    // Mutex: block thread to access until it is available
    fn withdraw(the_bank: &Arc<Mutex<Bank>>, amt: f32) {
        let mut bank_ref = the_bank.lock().unwrap();
        if bank_ref.balance < 5.00 {
            println!("Current balance = {}, please withdraw a smaller amount.", bank_ref.balance);
        } else {
            bank_ref.balance -= amt;
            println!("Cusotmer withdrew = {}, Current balance = {}", amt, bank_ref.balance);
        }
    }

    fn customer(the_bank: Arc<Mutex<Bank>>) {
        withdraw(&the_bank, 5.0);
    }

    let mut bank = Arc::new(Mutex::new(Bank{balance: 100.0}));
    let handles = (0..10).map(|_| {
            let bank_ref = bank.clone();
            thread::spawn(|| {
                customer(bank_ref)
            })
        } 
    );
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Total : {}", bank.lock().unwrap().balance);

    
}
