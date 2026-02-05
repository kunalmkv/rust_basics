// fn main() {
//     let mut x = 5;
//     x = x + 1; // 6

//     let y = &mut x; // y is reference to the value of x, value of x is mutable
//     *y = *y + 1; // 7

//     println!("x={}", x);
//     println!("y={}", *y);
// }



fn main() {
    let mut x = 5;
    x = x + 1; // 6

    {
        let y = &mut x; // y is reference to the value of x, value of x is mutable
        y = y + 1; // 7
        println!("y={}", y);
    } // y goes out of scope here; mutable borrow ends

    println!("x={}", x); // now we can use x again
}