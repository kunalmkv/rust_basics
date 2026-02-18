#[derive(Debug)]
enum Shape {
    Circle,
    Rectangle,
}

impl Shape {
    fn new_circle() -> Self {
        Self::Circle
    }
}

fn main() {
    let circle: Shape = Shape::new_circle();
    println!("Circle:{:?}", circle);
}



enum Shapes2 {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}
 
impl Shapes2 {

    fn declare_circle(radius: f64) -> Self {
        Self::Circle(radius)
    }

    fn declare_rectangle(length: f64, width: f64) -> Self {
        Self::Rectangle(length, width)
    }

    fn declare_triangle(a: f64, b: f64, c: f64) -> Self {
        Self::Triangle(a, b, c)
    }
}

fn main() {
    let circle = Shapes2::declare_circle(10.0);
}