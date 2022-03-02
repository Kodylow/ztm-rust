// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    age: i32,
    name: String,
    favorite_color: String
}
fn main() {
    let a = Person {
        age: 1,
        name: String::from("Kody"),
        favorite_color: String::from("Orange")
    };

    let b = Person {
        age: 2,
        name: String::from("Ali"),
        favorite_color: String::from("Pink")
    };

    let c = Person {
        age: 1,
        name: String::from("Rory"),
        favorite_color: String::from("Green")
    };

    let people = vec![a, b, c];

    for p in people {
        if p.name == "Kody" {
            println!("{:?}", p.name);
            println!("{:?}", p.age);
            println!("{:?}", p.favorite_color);
        } else {
            println!("Not Kody");
        }
    }
}
