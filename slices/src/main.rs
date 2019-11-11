fn main() {
    let mut arr: [i32; 4] = [1,2,3,4];
    // lexical
    for i in &mut arr {
        *i += 1;
    }
    // &mut arr ends here
    println!("{:?}", arr);
    let (x, y) = arr.split_at_mut(1);
    // x = [1], y = [2,3,4]
    let x1 = &mut x[0];
    let (y1, y2) = y.split_at_mut(1);
    // x1 = &x[1], y1 = [2], y2 = [3,4]
    let y3 = &mut y1[0];
    *y3 *= 3;
    *x1 *= 55;
    //end y3, x1, etc.
    // &arr
    println!("{} {}", arr[0], arr[1]);
}
