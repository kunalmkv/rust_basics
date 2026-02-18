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