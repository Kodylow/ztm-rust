// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn main() {
    let tup = (3, 4);
    let (x, y) = tup;

    println!("X of {:?}", x);

    if y < 5 {
        println!("Y of {:?} is less than 5", y);
    } else if y == 5 {
        println!("Y of {:?} is equal to 5", y);
    } else {
        println!("Y of {:?} is greater than 5", y);
    }
}
