#[derive(Debug)]
struct Rectangle {
    length: u8,
    breadth: u8,
}

impl Rectangle {
    // Associated function (constructor)
    /* 
    fn new(new_length: u8, new_breadth: u8) -> Rectangle {
        Rectangle {
            length: new_length,
            breadth: new_breadth,
        }
    }
    */

    fn new(new_length: u8, new_breadth: u8) -> Self {
        Self {
            length: new_length,
            breadth: new_breadth,
        }
    }

    // Method (takes self reference)
    fn area(&self) -> u8 {
        self.length * self.breadth
    }
}

// Regular function (not inside impl)
fn area_rec(rec: &Rectangle) -> u8 {
    rec.length * rec.breadth
}

fn main() {
    // Creating object using associated function
    let rec_one: Rectangle = Rectangle::new(10, 5);

    // Creating object using struct literal
    let rec_two = Rectangle {
        length: 5,
        breadth: 2,
    };

    println!("Area using normal function: {}", area_rec(&rec_one));
    println!("Area using method: {}", rec_two.area());
    println!("rec 2: {:?}", rec_two);
}