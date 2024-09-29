enum Shape {
    Rectangle(f64, f64),
    Circle(f64),
}

fn main() {
    let rect = Shape::Rectangle(2.0, 2.0);
    print_area(rect);
    let circle = Shape::Circle(2.0);
    print_area(circle);
}

fn print_area(shape: Shape) {
    let area = match shape {
        Shape::Rectangle(a, b) => a * b,
        Shape::Circle(r) => 3.14 * r * r,
    };
    println!("The area is: {}", area);
}
