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
