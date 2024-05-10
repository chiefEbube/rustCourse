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

// * Use a struct for a persons age, name, and favorite color
struct ResidentInfo {
    // * The color and name should be stored as a String
    age: i32,
    name: String,
    fav_color: String,
}

fn print_name_and_color(name: &str, fav_color: &str) {
    println!("name: {:?}", name);
    println!("favorite color: {:?}", fav_color);
}

fn main() {
    // * Create and store at least 3 people in a vector
    let residents = vec![
        ResidentInfo {
            age: 31,
            name: "Amaka".to_owned(),
            fav_color: String::from("red"),
        },
        ResidentInfo {
            age: 10,
            name: "Chidinma".to_owned(),
            fav_color: String::from("yellow"),
        },
        ResidentInfo {
            age: 5,
            name: "Mama".to_owned(),
            fav_color: String::from("white"),
        },
    ];

    // * Iterate through the vector using a for..in loop
    for resident in residents {
        // * Use an if expression to determine which person's info should be printed
        if resident.age <= 10 {
            // * The name and colors should be printed using a function
            print_name_and_color(&resident.name, &resident.fav_color);
        }
    }
}
