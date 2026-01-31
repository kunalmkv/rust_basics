// fn main() {
//     let x:u8 = 5;//x memory
//     process_integer(x);
//     println!("The value of x in main() is {}",x);
// }

// fn process_integer(item:u8){//item memory 5
//    println!("The value of x in process_integer() is {}",item);
// }



fn main() {
    let x:String = String::from("Hello");//x is the owner of Hello
    process_string(x);//transfer of ownership
    println!("The value of x in main() is {}",x);
}

fn process_string(item:String){//Hello-new owner is item
    println!("The value of x in process_string() is {}",item);
}

/* 
fn main() {
    let x: String = String::from("Hello"); // x is the owner of "Hello"
    process_string(&x); // borrow: pass a reference, x keeps ownership
    println!("The value of x in main() is {}", x); // x still valid here
}

fn process_string(item: &String) {
    // item borrows the string; does not take ownership
    println!("The value in process_string() is {}", item);
}
*/