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
    Orange,
    Yellow,
}

fn print_color_name(color: Colors) {
    match color {
        Colors::Red => println!("red"),
        Colors::Orange => println!("orange"),
        Colors::Yellow => println!("yellow"),
    }
}
fn main() {
    print_color_name(Colors::Red);
    print_color_name(Colors::Orange);
    print_color_name(Colors::Yellow);
}
