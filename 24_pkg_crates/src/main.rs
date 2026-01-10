mod math;
use math::*;

fn print_name(name:&str) {
  println!("print_name: {}",name);
}
fn main() {
    print_name("Shah Jabir");
    let result = add_by_num(5, 6);
    println!("add_by_num: {}", result);
    let result = add_by_number(-10, -25);
    println!("add_by_number: {}", result);
    let result = sub_by_num(20, 10);
    println!("sub_by_num: {}", result);
    let result = sub_by_number(-10, -15);
    println!("sub_by_number: {}", result);
}
