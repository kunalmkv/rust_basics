/*
fn main() {
    let bonus = 10;

    let add_bonus = |x: i32| x + bonus;

    println!("{}", add_bonus(5)); // 15
}
*/

fn add_bonus(x: i32, bonus: i32) -> i32 {
    x + bonus
}

fn main() {
    let bonus = 10;

    println!("{}", add_bonus(5, bonus)); // 
    println!("Bon {}", bonus); // 10
}