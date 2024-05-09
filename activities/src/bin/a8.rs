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

enum Flavor {
    Coffee,
    Tea,
    Whiskey,
    Soda
}

struct Drink {
    flavor: Flavor,
    ounce: f64,
}

fn print_drink(my_drink: Drink){
    match my_drink.flavor {
        Flavor::Coffee => println!("The drink flavor is coffee"),
        Flavor::Tea => println!("The drink flavor is tea"),
        Flavor::Whiskey => println!("The drink flavor is whiskey"),
        Flavor::Soda => println!("The drink flavor is soda")
    }

    println!("The drink has an ounce of {:?}", my_drink.ounce)
}

fn main() {
    let oge = Drink {
        flavor: Flavor::Coffee,
        ounce: 1.88,
    };

    let amy= Drink {
        flavor: Flavor::Tea,
        ounce: 1.99,
    };

    let chidi = Drink {
        flavor: Flavor::Whiskey,
        ounce: 2.00,
    };

    let ebube = Drink {
        flavor: Flavor::Soda,
        ounce: 2.11,
    };

    print_drink(oge);
    print_drink(amy);
    print_drink(chidi);
    print_drink(ebube);
}
