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

/*
The stack is very fast, and is where memory is allocated in Rust by default. 
But the allocation is local to a function call, and is limited in size. 
The heap, on the other hand, is slower, and is explicitly allocated by your program. 
But it's effectively unlimited in size, and is globally accessible.

Box is a very convenient type in Rust. 
When you use a Box , you can put a type on the heap instead of the stack.
 */

pub fn smart_pointer_example() {
    // box
    let b_int1 = Box::new(10);
    println!("b_int1 = {}", b_int1);
    struct TreeNode<T> {
        //Option allows there is no TreeNode at a child
        pub left: Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
        pub key: T,
    }
    impl<T> TreeNode<T> {
        pub fn new(key: T) -> Self {
            TreeNode {
                left:None, 
                right: None, 
                key
            }
        }
        pub fn left(mut self, node: TreeNode<T>) -> Self {
            self.left = Some(Box::new(node));
            self
        }
        pub fn right(mut self, node: TreeNode<T>) -> Self {
            self.right = Some(Box::new(node));
            self
        }
    }
    // Build a tree with one left and right rode
    let node1 = TreeNode::new(1)
            .left(TreeNode::new(2))
            .right(TreeNode::new(3));

}