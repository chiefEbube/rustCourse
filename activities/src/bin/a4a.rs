// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display

fn main() {
    // * Use a variable set to either true or false
    let some_bool = false;

    // * Use a match expression to determine which message to display
    match some_bool {
        true => println!("It's true"),
        false => println!("It's false")
    }
}
