use std::mem;
use std::collections::HashSet;
#[cfg(test)]
#[macro_use]
extern crate quickcheck;

// Write iterative insertion sort
fn iterative_insert(arr: &mut Vec<u32>) -> () {
    
}

// Write recursive insertion sort
fn recursive_insert(arr: &mut Vec<u32>) -> () {
    
}

fn main() {
    let mut a = vec![2,3,1,2,3,333,];
    let mut b = a.clone();
    iterative_insert(&mut a);
    recursive_insert(&mut b);
    println!("{:?}", a);
    println!("{:?}", b);
}

fn sorted(arr: &[u32]) -> bool {
    if arr.len() < 2 { true }
    else {
        let (_, rest) = arr.split_at(1);
        arr[0] <= arr[1] && sorted(rest)
    }
}

fn count(arr: &[u32], x: u32, acc: u32) -> u32 {
    if arr.len() == 0 { acc }
    else {
        let (_, rest) = arr.split_at(1);
        let z = if x == arr[0] { 1 } else { 0 };
        count(rest, x, acc+z)
    }
}

fn same_elems(arr1: &[u32], arr2: &[u32]) -> bool {
    let elems: HashSet<u32> = arr1.iter().cloned().collect();
    for x in elems {
        if count(arr1,x,0) != count(arr2,x,0) {
            return false;
        }
    }
    true
}

fn correct(srtd: &[u32], orig: &[u32]) -> bool {
    srtd.len() == orig.len() && sorted(srtd) && same_elems(srtd, orig)
}


#[cfg(test)]
quickcheck! {
    fn prop(xs: Vec<u32>) -> bool {
        let mut x1 = xs.clone();
        let mut x2 = xs.clone();
        recursive_insert(&mut x1);
        iterative_insert(&mut x2);
        correct(x1.as_slice(), xs.as_slice()) &&
        correct(x2.as_slice(), xs.as_slice())
    }
}
