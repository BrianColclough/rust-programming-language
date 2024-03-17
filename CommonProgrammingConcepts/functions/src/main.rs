fn main() {
    println!("Hello, world!");
    
    another_function();
    let y = return_a_value();
    println!("the value of y is: {y}")
}

fn another_function() {
    println!("Another function.");
}

fn return_a_value() -> i32 {
    5
}
