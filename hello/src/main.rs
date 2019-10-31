mod func;
use crate::func::fact::{fact};
use crate::func::fib::{fibs};
use crate::func::func_v;

use std::env;

fn main() {
    let mut n = 5;
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        n = args[1].trim().parse().unwrap_or(5);
    }
    
    let mut v = vec![];
    func_v(n, &mut v, &fact);
    func_v(n, &mut v, &fibs);
    for x in v {
        print!("{}, ", x);
    }
    println!();
}


#[test]
fn test1() {
    assert_eq!(fact(5), 120);
}
