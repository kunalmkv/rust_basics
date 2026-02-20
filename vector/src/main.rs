fn main() {
    let mut _v1: Vec<i32> = Vec::new();
    let mut _v2 = Vec::<i32>::new();
    let mut v3 = vec![100, 101, 102];

    // Index into a vector safely, return an Option<T>
    let opt = v3.get(10);

    match opt {
        Some(value) => println!("Value: {}", value),
        None => println!("No value"),
    }

    // Push elements onto the vector
    v3.push(1);
    v3.push(5);

    // Pop the last element from the vector
    v3.pop();

    // Insert an element at a specific index
    v3.insert(0, 99); 

    // Iterate over the elements of the vector and print each one
    for item in &v3 {
        println!("{}", item);
    }
    println!("v3={:?}", &v3);
}