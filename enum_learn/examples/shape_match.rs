enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
}

const PI: f64 = 3.14;

impl Shape {
    fn new_circle(radius: f64) -> Self {
        Self::Circle(radius)
    }

    fn new_rectangle(len: f64, bre: f64) -> Self {
        Self::Rectangle(len, bre)
    }

    fn area(&self) {
        match self {
            Shape::Circle(radius) => {
                println!("Area of circle: {}", PI * radius * radius)
            }
            Shape::Rectangle(len, bre) => {
                println!("Area of rectangle: {}", len * bre)
            }
        }
    }
}

fn main() {
    let circle: Shape = Shape::new_circle(5.0);
    let rec: Shape = Shape::new_rectangle(10.0, 5.0);

    circle.area();
    rec.area();
}
