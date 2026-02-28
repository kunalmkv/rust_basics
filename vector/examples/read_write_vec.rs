fn main() {
    let mut vrr: Vec<&str> = vec!["Hello", "World"];

    read_arr(&vrr);
    write_arr(&mut vrr);

    println!("{:?}", vrr);
}

fn read_arr(vrr: &[&str]) {
    println!("{:?}", vrr);
}

fn write_arr(vrr: &mut Vec<&str>) {
    vrr.push("Rust");
}
