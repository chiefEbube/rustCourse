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

fn print_message(some_bool: bool){
    match some_bool {
        true => println!("it's small"),
        false => println!("it's big")
    }
}
fn main() {
    let my_var = 101;
    let is_small = my_var <= 100;

    print_message(is_small)
}
