
// Define a trait for shapes
trait Shape {
    fn area(&self) -> f64;
}

// Implement the trait for Circle
struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// Implement the trait for Rectangle
struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// Function to print the area of any shape
fn print_area(shape: &impl Shape) {
    println!("Area: {}", shape.area());
}

fn main() {
    // Create instances of Circle and Rectangle
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle { width: 3.0, height: 4.0 };

    // Use the print_area function with different shapes
    print_area(&circle);
    print_area(&rectangle);
}