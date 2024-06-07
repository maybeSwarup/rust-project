enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64),
}

fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Square(length) => length * length,
        Shape::Rectangle(width, length) => width * length,
    }
}

fn main() {
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.0, 6.0);

    println!("Area of circle: {}", calculate_area(circle));
    println!("Area of circle: {}", calculate_area(square));
    println!("Area of circle: {}", calculate_area(rectangle));
}
