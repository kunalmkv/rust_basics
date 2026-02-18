fn main() {
    let user_id_1: i32 = 1;
    let user_id_2: i32 = 2;

    let value_1: Option<i32> = get_user_phone_number(user_id_1);
    let value_2: Option<i32> = get_user_phone_number(user_id_2);

    println!("Value 1: {:?}", value_1);
    println!("Value 2: {:?}", value_2);
}

fn get_user_phone_number(user_id: i32) -> Option<i32> {
    let mob_num: i32 = 923233;

    if user_id == 1 {
        return Some(mob_num);
    } else {
        return None;
    }
}