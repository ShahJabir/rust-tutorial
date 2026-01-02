fn main() {
  let s1:String = String::from("Hello");
  let len = calculate_length(&s1);
  println!("The length of string {} is {}", s1, len);
}

fn calculate_length(s:&String) -> usize {
  return s.len();
}
