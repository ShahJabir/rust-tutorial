use std::fmt::Display;

fn print_data<T:Display>(data:T){
  println!("Data: {}", data)
}
fn main() {
    let x = 5;
    let y = true;
    let z = "Hello".to_owned();
    print_data(x);
    print_data(y);
    print_data(z);
}
