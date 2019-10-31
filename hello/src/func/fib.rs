pub fn fibs(n: u64) -> u64{
    if n < 2 { 1 } else { fibs(n-1) + fibs(n-2) }
}

#[test]
fn test1() {
    assert_eq!(fibs(0), 1);
    assert_eq!(fibs(5), 8);
}
