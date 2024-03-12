fn main() {
    let x = 5;
    println!("The value of x is: {x}");

    // cannot assign twice to immutable variable
    // x = 6;

    println!("The value of x is: {x}");

    // mutable variables
    let mut y = 5;
    println!("The value of y is: {y}");
    y = 6;
    println!("The value of y is: {y}");
}
