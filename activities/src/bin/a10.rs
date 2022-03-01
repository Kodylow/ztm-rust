// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

use std::cmp::Ordering;

fn print_more(x: i32) {
    println!("{:?} is greater than or equal to 100", x);
}

fn print_less(x: i32) {
    println!("{:?} is less than 100", x)
}

fn main() {
    let x = 80;
    match x.cmp(&100) {
        Ordering::Less => print_less(x),
        _ => print_more(x)
    }
}
