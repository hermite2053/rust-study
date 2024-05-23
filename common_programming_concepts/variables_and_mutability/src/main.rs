fn main() {
    let x = 1;
    let mut y = 7;
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    // x = 6;  // should result in a compile error.
    y = 8;
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
}
