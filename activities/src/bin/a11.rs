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

struct GroceryItem {
    quantity: i32,
    id: i32,
}

fn dispaly_quantity(grocery_item: &GroceryItem) {
    println!("Quantity: {:?}", grocery_item.quantity);
}

fn display_id(grocery_item: &GroceryItem) {
    println!("Id: {:?}", grocery_item.id);
}

fn main() {
    let mixer = GroceryItem {
        quantity: 132,
        id: 10
    };

    dispaly_quantity(&mixer);
    display_id(&mixer);
}
