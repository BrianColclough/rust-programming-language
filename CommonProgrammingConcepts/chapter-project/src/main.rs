use std::io;

fn main() {
    println!("Enter a any number to get the nth fibonacci number: ");

    let mut fib_num = String::new();

    io::stdin()
        .read_line(&mut fib_num)
        .expect("failed to read your number");

    // if parse suceeds we return the number and if it fails we return 0
    let num = match fib_num.trim().parse::<i32>() {
        Ok(num) => num,
        Err(_) => 0
    };
    
    println!("Your fibonacci number is {}", nth_fib_number(num));
}

fn nth_fib_number(n: i32) -> i32 {
    if n == 0 {
        return 0
    } else if n == 1{
        return 1
    }else {
        return nth_fib_number(n - 1) + nth_fib_number(n-2);
    }
}
