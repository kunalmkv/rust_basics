fn main() {
    let x = 10;
    let y = get_val();

    println!("x+y:{}", x + y);
}
fn get_val() -> i32 {
    let y = 5;
    y
}
