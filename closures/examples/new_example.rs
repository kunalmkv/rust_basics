fn main() {
    let x: String = String::from("hello");
    let consume_and_return_x = || x;
    //println!("{}", x);
    let y: String = consume_and_return_x(); //ownership of value x gets transferred to y
    println!("{}", y);

    let z: String = y; //ownership of hello gets transferred to z
    println!("{}", z);
}