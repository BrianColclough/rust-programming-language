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
            printl!("remaining: {remaining}");
            if remaining == 0 {
                break;
            }
            if count == 2{
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
}

