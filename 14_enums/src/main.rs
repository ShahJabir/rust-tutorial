#[derive(Debug)]
enum Shape {
    Circle(f32),
    Rectangle(f32, f32),
    Square(f32),
    Triangle(f32, f32),
    Pentagon(f32),
    Hexagon(f32),
}

impl Shape {
    fn new_circle(radius: f32) -> Self {
        Self::Circle(radius)
    }
    
    fn new_rectangle(height: f32, width: f32) -> Self {
        Self::Rectangle(height, width)
    }
    
    fn new_square(side: f32) -> Self {
        Self::Square(side)
    }
    
    fn new_triangle(base: f32, height: f32) -> Self {
        Self::Triangle(base, height)
    }
    
    fn new_pentagon(side: f32) -> Self {
        Self::Pentagon(side)
    }
    
    fn new_hexagon(side: f32) -> Self {
        Self::Hexagon(side)
    }
    
    fn area(&self) -> f32 {
        match self {
            Shape::Circle(radius) => 3.14159 * radius * radius,
            Shape::Rectangle(height, width) => height * width,
            Shape::Square(side) => side * side,
            Shape::Triangle(base, height) => 0.5 * base * height,
            Shape::Pentagon(side) => 1.72048 * side * side,
            Shape::Hexagon(side) => 2.59808 * side * side,
        }
    }
    
    fn perimeter(&self) -> f32 {
        match self {
            Shape::Circle(radius) => 2.0 * 3.14159 * radius,
            Shape::Rectangle(height, width) => 2.0 * (height + width),
            Shape::Square(side) => 4.0 * side,
            Shape::Triangle(base, height) => {
                let hypotenuse = (base * base + height * height).sqrt();
                base + height + hypotenuse
            }
            Shape::Pentagon(side) => 5.0 * side,
            Shape::Hexagon(side) => 6.0 * side,
        }
    }
    
    fn name(&self) -> &str {
        match self {
            Shape::Circle(_) => "Circle",
            Shape::Rectangle(_, _) => "Rectangle",
            Shape::Square(_) => "Square",
            Shape::Triangle(_, _) => "Triangle",
            Shape::Pentagon(_) => "Pentagon",
            Shape::Hexagon(_) => "Hexagon",
        }
    }
}

fn main() {
    // Create different shapes
    let circle = Shape::new_circle(5.0);
    let rectangle = Shape::new_rectangle(10.0, 20.0);
    let square = Shape::new_square(8.0);
    let triangle = Shape::new_triangle(12.0, 9.0);
    let pentagon = Shape::new_pentagon(6.0);
    let hexagon = Shape::new_hexagon(7.0);
    
    // Store all shapes in a vector
    let shapes = vec![circle, rectangle, square, triangle, pentagon, hexagon];
    
    // Print information for each shape
    println!("========== SHAPE CALCULATIONS ==========\n");
    
    for (index, shape) in shapes.iter().enumerate() {
        println!("Shape #{}: {}", index + 1, shape.name());
        println!("  Details: {:?}", shape);
        println!("  Area: {:.2}", shape.area());
        println!("  Perimeter: {:.2}", shape.perimeter());
        println!();
    }
    
    // Calculate total area
    let total_area: f32 = shapes.iter().map(|s| s.area()).sum();
    println!("========================================");
    println!("Total Area of All Shapes: {:.2}", total_area);
    println!("========================================");
}
