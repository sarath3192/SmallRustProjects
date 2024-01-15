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

fn print_areas(shapes: Vec<&dyn Shape>) {
    for shape in shapes {
        println!("Area: {}", shape.area());
    }
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle { width: 3.0, height: 4.0 };

    let shapes: Vec<&dyn Shape> = vec![&circle, &rectangle];

    print_areas(shapes);
}
