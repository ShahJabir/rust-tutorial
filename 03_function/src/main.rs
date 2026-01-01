fn add(num1:u16, num2:u16) -> u32 {
  (num1 as u32) + (num2 as u32)
}
fn main() {
    let num1:u16 = 65535;
    let num2:u16 = 65535;
    let result:u32 = add(num1, num2);
    println!("{}", result)
}
