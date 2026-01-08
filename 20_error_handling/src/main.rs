use std::fs::File;
use std::io::{self, Read};

// Function that returns Result
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

// Function that returns Option
fn find_user(id: u32) -> Option<String> {
    match id {
        1 => Some(String::from("Shah Jabir")),
        2 => Some(String::from("Abdul Rahim")),
        3 => Some(String::from("Ahmed Ali")),
        _ => None,
    }
}

// Function with file operations (returns Result)
fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;  // ? operator for error propagation
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    println!("========== BASIC ERROR HANDLING ==========\n");
    
    // 1. Handling Result with match
    println!("1. Division Examples:");
    match divide(10.0, 2.0) {
        Ok(result) => println!("   10 / 2 = {}", result),
        Err(e) => println!("   Error: {}", e),
    }
    
    match divide(10.0, 0.0) {
        Ok(result) => println!("   10 / 0 = {}", result),
        Err(e) => println!("   Error: {}", e),
    }
    
    // 2. Handling Option with match
    println!("\n2. User Search Examples:");
    match find_user(1) {
        Some(name) => println!("   Found user: {}", name),
        None => println!("   User not found"),
    }
    
    match find_user(999) {
        Some(name) => println!("   Found user: {}", name),
        None => println!("   User not found"),
    }
    
    // 3. Using unwrap_or and unwrap_or_else
    println!("\n3. Using unwrap_or:");
    let result = divide(10.0, 2.0).unwrap_or(0.0);
    println!("   Result: {}", result);
    
    let result = divide(10.0, 0.0).unwrap_or(0.0);
    println!("   Result (with error): {}", result);
    
    // 4. Using if let
    println!("\n4. Using if let:");
    if let Some(name) = find_user(2) {
        println!("   User found: {}", name);
    }
    
    if let Ok(result) = divide(20.0, 4.0) {
        println!("   Division result: {}", result);
    }
    
    // 5. File reading with error handling
    println!("\n5. File Reading:");
    match read_file("test.txt") {
        Ok(contents) => println!("   File contents: {}", contents),
        Err(e) => println!("   Error reading file: {}", e),
    }
}
