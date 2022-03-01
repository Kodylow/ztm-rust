// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct Grocery {
    id: i32,
    quantity: i32
}

fn display_q (x: &Grocery) {
    println!("quantity: {:?}", x.quantity);
}

fn display_id (x: &Grocery) {
    println!("id: {:?}", x.id)
}

fn main() {
    let apples = Grocery {
        id: 1,
        quantity: 10
    };
    display_q(&apples);
    display_id(&apples);
}
