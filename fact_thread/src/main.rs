use std::sync::{Arc, Mutex};
use std::thread;

fn fib(lb: u32, ub: u32, n: Arc<Mutex<u32>>) {
    // [lb, ub], [lb, ub)
    for i in lb..ub {
        *n.lock().unwrap() *= i;
    }
}

fn main() {
    // &mut u32 is not sync
    // Mutex<u32> is sync but not send
    // Arc<Mutex<u32>> ~ &Mutex<u32> is send + sync
    let n = Arc::new(Mutex::new(1));
    let n_c1 = n.clone();
    let t1 = thread::spawn(move || {fib(1,5,n_c1);});
    let n_c2 = n.clone();
    let t2 = thread::spawn(move || {fib(5,10,n_c2);});
    t1.join();
    t2.join();
    println!("{}", *n.lock().unwrap());
    //println!("{}", *n);
}
