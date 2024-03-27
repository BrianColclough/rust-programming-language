use std::{collections::HashMap, io};

fn main() {
    let mut map = HashMap::new();
    loop {
        if false {
            break;
        }
        println!("pick an option:");
        println!("1. Add an employee to a department");
        println!("2. List all employees in a department");
        let mut user_input = String::new();
        match io::stdin().read_line(&mut user_input){
                Ok(..) => handle_user_input(user_input, &mut map),
                err => println!("your input is not valid {:#?}", err),
            }
    }
}

fn handle_user_input(user_input: String, map: &mut HashMap<String, String>){
    if user_input == "1" {
        println!("what is this emplyees name?");
        let mut employee_name = String::new();
        io::stdin().read_line(&mut employee_name);

        println!("what is this emplyees department?");
        let mut department = String::new();
        match io::stdin().read_line(&mut employee_name){
                Ok(..) => *map.entry(department).or_insert(employee_name),
                err => println!("your input is not valid {:#?}", err),
            }

    } else if user_input == "2"{}
}
