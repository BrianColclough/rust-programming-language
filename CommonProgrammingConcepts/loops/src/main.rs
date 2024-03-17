fn main() {
    println!("Hello, world!");
    let mut counter = 0;

    // returning a value from a loop
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {result}");

    // break and continue statements usually apply to the innermost loop
    // but you can specigy a loop label and use it with break and continue
    // to break out of the non inner most loop

    let mut count = 0;

    'counting_up: loop {
        println!("counting up");
        let mut remaining = 10;

        loop {
            println!("remaining: {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    //while loops
    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!!!");


    // conditional while loops
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // a more concice alternaitive is to use a for loop
    for element in a {
        println!("the value is: {element}");
    }

    // this increases the safety of the code and eliminates bugs that could arise from going out of
    // the bounds of the array

    //  if we were to use a for loop to make the range example from before
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}
