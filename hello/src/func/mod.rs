pub mod fact;
pub mod fib;


pub fn func_v(n: u64, v: &mut Vec<u64>, f: dyn Fn(u64) -> u64) {
    for i in 0..=n {
        v.push(f(i));
    }
}
