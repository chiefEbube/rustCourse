// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
enum Color {
    White,
    Brown
}

impl Color {
    fn print(&self){
        match self {
            Color::Brown => println!("brown"),
            Color::White => println!("White")
        }
    }
}
struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions {
    fn print(&self){
        println!("Width: {:?}", self.width);
        println!("Height: {:?}", self.height);
        println!("Depth: {:?}", self.depth);
    }
}
struct ShippingBox {
    weight: f64,
    color: Color,
    dimensions: Dimensions,
}

impl ShippingBox {
// * Implement functionality on the box struct to create a new box
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            weight,
            color,
            dimensions,
        }
    }

// * Implement functionality on the box struct to print the characteristics
    fn print(&self){
        self.color.print();
        self.dimensions.print();
        println!("Weight: {:?}", self.weight)
    }

}
fn main() {
    let my_dimensions = Dimensions{
        width: 2.3,
        height: 2.3,
        depth: 4.0
    };

    let my_box = ShippingBox::new(5.0, Color::Brown, my_dimensions);
    my_box.print();
}
