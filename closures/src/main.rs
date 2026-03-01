fn main() {
    let mut counter: i32 = 0;

    let mut increase_counter = || {
        counter = counter + 1;
        println!("{}", counter);
    };

    increase_counter(); // 1
    increase_counter(); // 2
    increase_counter(); // 3
}