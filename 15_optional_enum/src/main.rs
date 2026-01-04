fn main() {
    let user_id_1:i8 = 1;
    let user_id_2:i8 = 2;

    match get_user_phone_number(user_id_1) {
        Some(data) => println!("User 1 Data: {}", data),
        None => println!("User 1 mob num does not exist")
    }

    match get_user_phone_number(user_id_2) {
        Some(data) => println!("User 2 Data: {}", data),
        None => println!("User 2 mob num does not exist")
    }
}

fn get_user_phone_number(user_id:i8) -> Option<i32> {
  let mob_num = 923233;
  if user_id == 1 {
    return Some(mob_num);
  } else {
    return None;
  }
}
