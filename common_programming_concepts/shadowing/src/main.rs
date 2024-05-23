fn main() {
    let x = 9;
    let x = x + 1;  // shadowing

    {
        let x = x * 10;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
