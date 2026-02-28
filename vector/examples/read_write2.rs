fn main() {
    let mut vrr: Vec<&str> = vec!["Hello", "World"];

    // Read the original vector
    read_arr_one(&vrr);

    // Modify the vector
    write_arr_one(&mut vrr);

    // Print the modified vector
    println!("{:?}", vrr);
}

fn read_arr_one(vrr1: &[&str]) {
    // Print the slice of string references
    println!("Read Arr One: {:?}", vrr1);

    // Call another function to read the same slice
    read_arr_two(vrr1);
}

fn read_arr_two(vrr2: &[&str]) {
    // Print the slice of string references
    println!("Read Arr Two: {:?}", vrr2);
}

fn write_arr_one(vrr3: &mut Vec<&str>) {
    // Add a new string slice to the vector
    vrr3.push("New");

    // Call another function to modify the vector
    write_arr_two(vrr3);
}

fn write_arr_two(vrr4: &mut Vec<&str>) {
    // Add another string slice to the vector
    vrr4.push("Few");
}