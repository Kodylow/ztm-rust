// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavors {
    Cherry,
    Cola,
}

struct Drink {
    flavor: Flavors,
    size: i32
}

fn main() {
    let a = Drink {
        flavor: Flavors::Cherry,
        size: 16
    };
    match a.flavor {
        Flavors::Cherry => println!("Cherry"),
        Flavors::Cola => println!("Cola")
    };
    println!("{:?}", a.size);
    let b = Drink {
        flavor: Flavors::Cola,
        size: 12
    };
    match b.flavor {
        Flavors::Cherry => println!("Cherry"),
        Flavors::Cola => println!("Cola")
    };
    println!("{:?}", b.size);
}
