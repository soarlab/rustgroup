pub fn fact(n: u64) -> u64 {
    if n < 2 { 1 } else { n*fact(n-1) }
}

#[test]
fn test1() {
    assert_eq!(fact(5), 120);
}
