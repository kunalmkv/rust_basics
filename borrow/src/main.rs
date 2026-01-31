fn main() {
    let mut s1: String = String::from("Hello");
    append_string(&mut s1);
    println!("The new string is {}",s1);

    // let len:usize = calculate_length(&s1);//borrow operation
    // println!("The length of {} is {}",s1,len);
}

// fn calculate_length(s2:&String)->usize{
//     return s2.len();
// }

fn append_string(s3: &mut String) {
    s3.push_str("World");
}
