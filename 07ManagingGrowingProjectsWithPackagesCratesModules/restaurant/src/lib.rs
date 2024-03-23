// use keyword
mod front_of_house;

use crate::front_of_house::hosting;

pub fn eat_at_restaurant3() {
    hosting::add_to_waitlist();
}


// crate
//  └── front_of_house
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//          └── take_payment
//

// pub fn eat_at_resuaurant() {
//     // Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();
//
//     // Relative path
//     front_of_house::hosting::add_to_waitlist();
// }

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // Change our mind about what breat we'd like
    meal.toast = String::from("Whole Wheat");
    println!("I'd like {} toast please", meal.toast);

    // we cannot access seasonal_fruit because it is private
    // meal.seasonal_fruit = String::from("blueberries");
}

mod back_of_house2 {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant2() {
    let _order1 = back_of_house2::Appetizer::Soup;
    let _order2 = back_of_house2::Appetizer::Salad;
}


// random leetcode question
// pub fn is_palindrome(x: i32) -> bool {
//     let num_string = x.to_string();
//     let reverse_string = num_string.chars().rev().collect::<String>();
//
//     let mut half = num_string.len() / 2;
//     if num_string.len() % 2 != 0 {
//         half += 1;
//     }
//
//     let char_indicies = num_string.char_indices();
//
//     for (i, c) in char_indicies {
//         if i == half {
//             break;
//         }
//         if c != reverse_string.chars().nth(i).unwrap() {
//             return false;
//         }
//     }
//
//     true
// }
//
//
