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

    match number {
        n if n < 0 => println!("The number is negative."),
        0 => println!("The number is Zero"),
        1 => println!("The number is One"),
        2 => println!("The number is Two"),
        3 => println!("The number is Three"),
        4 => println!("The number is Four"),
        5 => println!("The number is Five"),
        6 => println!("The number is Six"),
        7 => println!("The number is Seven"),
        8 => println!("The number is Eight"),
        9 => println!("The number is Nine"),
        10 => println!("The number is Ten"),
        n if n >= 11 && n <= 50 => println!("The number is between 11 to 50"),
        n if n >= 51 && n <= 100 => println!("The number is between 51 to 100"),
        _ => println!("This number is greater then 100")
    }
}
