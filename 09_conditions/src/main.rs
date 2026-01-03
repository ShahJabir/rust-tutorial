use std::io;
fn main() {
    let mut input_text: String = String::new();

    println!("Please input a number:");
    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read line");

    // Trim whitespace and parse the string to an i32 (32-bit integer)
    let number: i32 = input_text
        .trim()
        .parse()  // Converts the string to the specified type T
        .expect("Invalid input");           // Handle errors if the input isn't a valid number

    if number % 3 == 0 && number % 4 == 0 {
      println!("The number is divisible by both 3 & 4");
    } else if number % 3 == 0 {
        println!("The number is divisible by 3 but not by 4")
    } else if number % 4 == 0 {
        println!("The number is divisible by 4 but not by 3")
    } else {
        println!("The number is not divisible by both 3 & 4");
    }
}
