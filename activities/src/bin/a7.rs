// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Colors {
    Red,
    Blue,
    Green
    
}

fn main() {

    let color = Colors::Red;
    let ocolor = Colors::Green;
    let finalcolor = Colors::Blue;

    let a = [color, ocolor, finalcolor];

    for i in a {
        match i {
            Colors::Red => println!("red"),
            Colors::Blue => println!("blue"),
            Colors::Green => println!("green")
        };
    };
}
