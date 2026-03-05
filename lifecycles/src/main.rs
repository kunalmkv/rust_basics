fn main() {
    let r: &mut i32;
    let mut x = 5;
    r = &mut x;
    x = x + 1;
    println!("{}", r);
}
