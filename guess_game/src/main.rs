use std::io;
use rand::prelude::*;
use rand::rng;

fn main() {
    let guess_list = ["grapes", "banana", "orange"];
    let mut rng = rng();

    let index = rng.random_range(0..guess_list.len());
    let random_fruit = guess_list[index];
    println!("random_fruit:{}", random_fruit);

    let mut input = String::new();

    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let fruit_selected = input.trim().to_lowercase();
                println!("Fruit Selected:{}", fruit_selected);

                if !guess_list.contains(&fruit_selected.as_str()) {
                    println!("Fruit entered does not found!");
                    continue;
                }

                if guess_checker(&fruit_selected, random_fruit) {
                    println!("You are winner!");
                    break;
                } else {
                    println!("Retry!");
                    input.clear();
                    continue;
                }
            }
            Err(error) => {
                println!("Error:{}", error);
            }
        }
    }
}

fn guess_checker(guessed_fruit: &str, random_selected: &str) -> bool {
    return guessed_fruit == random_selected;
}